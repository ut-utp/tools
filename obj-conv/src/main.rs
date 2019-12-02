//! TODO!

// TODO: forbid
#![warn(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_lifetimes,
    unused_comparisons,
    unused_parens,
    while_true
)]
// TODO: deny
#![warn(
    missing_debug_implementations,
    intra_doc_link_resolution_failure,
    missing_docs,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    rust_2018_idioms
)]
#![doc(test(attr(deny(warnings))))]
#![doc(html_logo_url = "")] // TODO!


use lc3_isa::{Addr, Instruction, Word};
use lc3_isa::util::MemoryDump;
use lc3_shims::memory::FileBackedMemoryShim;
use lc3_os::OS_IMAGE;

use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;
use std::io::Result as IoResult;

use clap::{App, AppSettings, Arg};

pub type Loadable = (Addr, Word);

pub mod file_formats;
use file_formats::{Lc3Tools, Lumetta, ObjFileFormat};

macro_rules! cargo_env {
    ($cargo_env_var:ident) => {
        env!(concat!("CARGO_", stringify!($cargo_env_var)))
    };
}

fn args() -> App<'static, 'static> {
    App::new(cargo_env!(PKG_NAME))
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(cargo_env!(PKG_VERSION))
        .author(cargo_env!(PKG_AUTHORS))
        .about(cargo_env!(PKG_DESCRIPTION))
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Input object file to convert.")
                .long("input")
                .value_names(&["FILE"])
                .number_of_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Output file.")
                .long("output")
                .value_names(&["FILE"])
                .number_of_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("without-os")
                .short("w")
                .help("Do not layer program on top of an OS.")
                .long("without-os")
        )
        .arg(
            Arg::with_name("custom-os")
                .short("c")
                .help("Custom memory dump to use as the OS. Overrides the default OS.")
                .long("custom-os")
                .value_names(&["FILE"])
                .number_of_values(1)
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Print out parsed object file.")
                .long("verbose")
        )
}

fn print_loadable(loadable: impl Iterator<Item = (Addr, Word)>) {
    loadable.for_each(|(a, w)| {
        if let Ok(insn) = Instruction::try_from(w) {
            println!("{:04X}: {}", a, insn);
        } else {
            println!("{:04X}: {:04X}", a, w);
        }
    })
}

trait If { fn t<F: FnOnce()>(&self, func: F); }

impl If for bool {
    fn t<F: FnOnce()>(&self, func: F) { if *self { func() } }
}

    // fn convert_to_mem_map<P: AsRef<Path>>(parsed: Self::Parsed, path: P, with_os: bool) -> IoResult<()> {
    //     let pairs: Vec<Loadable> = parsed.into();

    //     let mut mem: FileBackedMemoryShim;
    //     if with_os {
    //         let os_path: String = format!("lc3os.mem");
    //         mem = FileBackedMemoryShim::from_existing_file(&os_path).unwrap();
    //         mem.change_file(path);
    //     } else {
    //         mem = FileBackedMemoryShim::new(path);
    //     }

    //     for &(addr, word) in pairs.iter() {
    //         mem[addr] = word;
    //     }

    //     mem.flush();

    //     Ok(())
    // }

fn try_format<F: ObjFileFormat, P: Copy + AsRef<Path>>(path: P, verbose: bool) -> std::io::Result<F::Parsed> {
    if !F::file_matches_format(&mut File::open(path)?) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Can't parse with this object file format.",
        ));
    }

    let returned = F::parse(&mut File::open(path)?)?;

    println!("Parsed as {}:", F::NAME);
    verbose.t(|| println!("{}", returned));

    verbose.t(|| {
        let loadable_iter = returned.clone().into().into_iter();
        println!("As a loadable:");
        print_loadable(loadable_iter);
    });

    Ok(returned.into())
}

// fn try_convert<F: ObjFileFormat, P: Copy + AsRef<Path>>(
//     path_read: P,
//     path_write: P,
//     with_os: bool
// ) -> std::io::Result<()> {
//     if !F::file_matches_format(&mut File::open(path_read)?) {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             "Can't parse with this object file format.",
//         ));
//     }

//     let parsed = F::parse(&mut File::open(path_read)?)?;

//     let result = F::convert_to_mem_map(parsed, path_write, with_os)?;

//     Ok(result)
// }

//fn main() -> std::io::Result<()> {
//    let matches = args().get_matches();
//    let path = matches.value_of("input").expect("filename is required");
//
//    if let Err(_) = try_format::<Lc3Tools, _>(path) {
//        eprintln!("Failed to parse as {}; trying to parse as {}:", Lc3Tools::NAME, Lumetta::NAME);
//        try_format::<Lumetta, _>(path)?;
//    }
//
//    Ok(())
//}

enum OsStrategy<'a> {
    Default,
    Custom(&'a str),
    None,
}

impl<'a> OsStrategy<'a> {
    fn new(with_os: bool, custom_path: Option<&'a str>) -> Self {
        use OsStrategy::*;

        match (with_os, custom_path) {
            (false, _) => None,
            (true, Option::None) => Default,
            (true, Some(path)) => Custom(path),
        }
    }

    fn to_memory_dump(self) -> MemoryDump {
        use OsStrategy::*;

        match self {
            Default => OS_IMAGE.clone(),
            Custom(ref path) => FileBackedMemoryShim::from_existing_file(path).expect("a valid OS image").into(),
            None => MemoryDump::blank(),

        }
    }
}

fn main() -> IoResult<()> {
    let matches = args().get_matches();
    let input_path = matches.value_of("input").expect("input object file is required");
    let output_path = matches.value_of("output").expect("output object file is required");

    let verbose = matches.is_present("verbose");

    let with_os = !matches.is_present("without-os");
    let custom_os_path = matches.value_of("custom-os");

    let program: Vec<Loadable> = try_format::<Lc3Tools<'_>, _>(input_path, verbose)
        .map(|l| l.into_iter().collect())
        .or_else(|_|{
            eprintln!(
                "Failed to parse as {}; trying to parse as {}:",
                Lc3Tools::NAME,
                <&Lumetta>::NAME
            );
            try_format::<&Lumetta, _>(input_path, verbose)
                .map(|l| l.into_iter().collect())
        })?;

    let mut image: MemoryDump = OsStrategy::new(with_os, custom_os_path)
        .to_memory_dump();

    let _ = image.layer_loadable(program);

    FileBackedMemoryShim::with_initialized_memory(output_path, image).flush().map_err(|_| std::io::Error::last_os_error())
}
