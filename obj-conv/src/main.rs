use clap::{App, AppSettings, Arg};
use lc3_isa::{Addr, Instruction, Word};

use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;

pub type Loadable = (Addr, Word);

pub mod file_formats;
use file_formats::{ObjFileFormat, Lc3Tools, Lumetta};

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
}

fn print_loadable<'a>(loadable: impl Iterator<Item = &'a (Addr, Word)>) {
    loadable.for_each(|(a, w)| {
        if let Ok(insn) = Instruction::try_from(*w) {
            println!("{:04X}: {}", a, insn);
        } else {
            println!("{:04X}: {:04X}", a, w);
        }
    })
}

fn try_format<F: ObjFileFormat, P: Copy + AsRef<Path>>(path: P) -> std::io::Result<F::Parsed> {
    if !F::file_matches_format(&mut File::open(path)?) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Can't parse with this object file format.",
        ));
    }

    let parsed = F::parse(&mut File::open(path)?)?;

    println!("Parsed as {}:", F::NAME);
    println!("{}", parsed);

    let loadable: Vec<Loadable> = parsed.clone().into();
    println!("As a loadable:");
    print_loadable(loadable.iter());

    Ok(parsed)
}

fn main() -> std::io::Result<()> {
    let matches = args().get_matches();
    let path = matches.value_of("input").expect("filename is required");

    if let Err(_) = try_format::<Lc3Tools, _>(path) {
        eprintln!("Failed to parse as {}; trying to parse as {}:", Lc3Tools::NAME, Lumetta::NAME);
        try_format::<Lumetta, _>(path)?;
    }

    Ok(())
}
