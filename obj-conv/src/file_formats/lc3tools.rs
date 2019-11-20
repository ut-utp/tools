/// [`ObjFileFormat`](super::ObjFileFormat) implementation for
/// [LC3Tools](https://github.com/chiragsakhuja/lc3tools).

use super::{IoResult, Loadable, ObjFileFormat};
use lc3_isa::Word;

use std::fs::File;
use std::io::Read;
use std::fmt::{self, Display};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemEntry {
    word: Word,
    orig: bool,
    line: String,
}

impl MemEntry {
    fn new(word: Word, orig: bool, line: String) -> Self {
        Self { word, orig, line }
    }
}

impl Display for MemEntry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.orig {
            write!(fmt, "<orig: {:04X}>", self.word)?
        } else {
            write!(fmt, "<{:04X}>", self.word)?
        }

        writeln!(fmt, "  {}", self.line)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lc3ToolsObjFile {
    version: [u8; 2],
    memory_entries: Vec<MemEntry>,
}

impl Lc3ToolsObjFile {
    pub fn get_version(&self) -> u16 {
        LittleEndian::read_u16(&self.version)
    }
}

impl Display for Lc3ToolsObjFile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.memory_entries.iter().try_for_each(|m| Display::fmt(m, fmt))
    }
}

impl From<Lc3ToolsObjFile> for Vec<Loadable> {
    fn from(obj: Lc3ToolsObjFile) -> Self {
        let mut addr = 0x0000;
        obj.memory_entries.iter()
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lc3Tools;

impl Lc3Tools {
    const HEADER: [u8; 5] = [0x1c, 0x30, 0x15, 0xc0, 0x01];
    const TESTED_VERSION: [u8; 2] = [0x01, 0x01];
}

impl Lc3Tools {
    fn read_mem_entry(file: &mut File) -> IoResult<MemEntry> {
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
}

impl ObjFileFormat for Lc3Tools {
    type Parsed = Lc3ToolsObjFile;
    const NAME: &'static str = "an LC3Tools Object File";

    fn file_matches_format(file: &mut File) -> bool {
        let mut header_buffer: [u8; 5] = [0; 5];
        if let Ok(_) = file.read_exact(&mut header_buffer) {
            header_buffer == Self::HEADER
        } else {
            false
        }

    }

    fn parse(file: &mut File) -> IoResult<Self::Parsed> {
        if !Self::file_matches_format(file) {
            panic!("Incorrect header.");
        }

        let mut version: [u8; 2] = [0; 2];
        file.read_exact(&mut version)?;

        if version != Self::TESTED_VERSION {
            eprintln!(
                "Warning! Untested object file version. ({:?})",
                Self::TESTED_VERSION
            );
        }

        let mut memory_entries = Vec::<MemEntry>::new();

        while let Ok(mem_entry) = Self::read_mem_entry(file) {
            memory_entries.push(mem_entry);
        }

        Ok(Lc3ToolsObjFile {
            version,
            memory_entries,
        })
    }
}
