use byteorder::{LittleEndian, ReadBytesExt};
use clap::{App, AppSettings, Arg};
use lc3_isa::{Addr, Instruction, Word};

use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;

macro_rules! cargo_env {
    ($cargo_env_var:ident) => {
        env!(concat!("CARGO_", stringify!($cargo_env_var)))
    };
}

pub fn args() -> App<'static, 'static> {
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

pub struct Lc3ToolsObjFile {
    version: [u8; 2],
    memory_entries: Vec<MemEntry>,
}

struct MemEntry {
    word: Word,
    orig: bool,
    line: String,
}

impl MemEntry {
    fn new(word: Word, orig: bool, line: String) -> Self {
        Self { word, orig, line }
    }
}

const HEADER: [u8; 5] = [0x1c, 0x30, 0x15, 0xc0, 0x01];
const TESTED_VERSION: [u8; 2] = [0x01, 0x01];

// const INSN_SEPERATOR: [u8; 5] = [0x00, 0x11, 0x00, 0x00, 0x00];

fn read_mem_entry(file: &mut File) -> std::io::Result<MemEntry> {
    let word = file.read_u16::<LittleEndian>()?;
    let orig = file.read_u8()?;

    let orig = match orig {
        0 => false,
        1 => true,
        _ => panic!("Invalid orig field!"),
    };

    let str_len = file.read_u32::<LittleEndian>()?;
    let mut line = String::with_capacity(str_len as usize);

    file.take(str_len as u64).read_to_string(&mut line)?;

    Ok(MemEntry::new(word, orig, line))
}

fn print_mem_entries<'a>(mem_entries: impl Iterator<Item = &'a MemEntry>) {
    mem_entries.for_each(|m| {
        if m.orig {
            print!("<orig: {:04X}>", m.word);
        } else {
            print!("<{:04X}>", m.word)
        }

        println!("  {}", m.line);
    })
}

fn mem_entries_to_loadable<'a>(
    mem_entries: impl Iterator<Item = &'a MemEntry>,
) -> Vec<(Addr, Word)> {
    let mut addr = 0x0000;
    mem_entries
        .filter_map(|m| {
            if m.orig {
                addr = m.word;
                None
            } else {
                let a = addr;
                addr += 1;

                Some((a, m.word))
            }
        })
        .collect()
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

fn main() -> std::io::Result<()> {
    let matches = args().get_matches();

    let file = matches.value_of("input").expect("filename is required");
    let mut file = File::open(file)?;

    let mut header_buffer: [u8; 5] = [0; 5];
    file.read_exact(&mut header_buffer)?;

    if header_buffer != HEADER {
        panic!("Incorrect header.");
    }

    let mut version: [u8; 2] = [0; 2];
    file.read_exact(&mut version)?;

    if version != TESTED_VERSION {
        eprintln!(
            "Warning! Untested object file version. ({:?})",
            TESTED_VERSION
        );
    }

    let mut memory_entries = Vec::<MemEntry>::new();

    while let Ok(mem_entry) = read_mem_entry(&mut file) {
        memory_entries.push(mem_entry);
    }

    println!("Memory Entries:");
    print_mem_entries(memory_entries.iter());

    println!("\nLoadable:");
    let loadable = mem_entries_to_loadable(memory_entries.iter());
    print_loadable(loadable.iter());

    let _ = Lc3ToolsObjFile {
        version,
        memory_entries,
    };

    Ok(())
}
