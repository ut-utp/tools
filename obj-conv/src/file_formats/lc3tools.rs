//! [`ObjFileFormat`](super::ObjFileFormat) implementation for
//! [LC3Tools](https://github.com/chiragsakhuja/lc3tools).
use super::{IoResult, Loadable, ObjFileFormat};
use lc3_isa::Word;

use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A single memory entry in an [`Lc3ToolsObjFile`](Lc3ToolsObjFile).
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.orig {
            write!(fmt, "<orig: {:04X}>", self.word)?
        } else {
            write!(fmt, "<{:04X}>", self.word)?
        }

        writeln!(fmt, "  {}", self.line)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Object File type for [`Lc3Tools`](Lc3Tools).
pub struct Lc3ToolsObjFile<'a> {
    version: [u8; 2],
    memory_entries: Vec<MemEntry>,
    // pos: Option<Addr>,
    _p: PhantomData<&'a ()>,
}

impl Lc3ToolsObjFile<'_> {
    /// Returns the version of the encoding used for the object file that was
    /// parsed.
    pub fn get_version(&self) -> u16 {
        LittleEndian::read_u16(&self.version)
    }
}

impl Display for Lc3ToolsObjFile<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.memory_entries
            .iter()
            .try_for_each(|m| Display::fmt(m, fmt))
    }
}

impl From<Lc3ToolsObjFile<'_>> for Vec<Loadable> {
    fn from(obj: Lc3ToolsObjFile<'_>) -> Self {
        let mut addr = 0x0000;
        obj.memory_entries
            .iter()
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
/// [`ObjFileFormat`](super::ObjFileFormat) implementation for
/// [LC3Tools](https://github.com/chiragsakhuja/lc3tools).
pub struct Lc3Tools<'a>(PhantomData<&'a ()>);

impl Lc3Tools<'_> {
    const HEADER: [u8; 5] = [0x1c, 0x30, 0x15, 0xc0, 0x01];
    const TESTED_VERSION: [u8; 2] = [0x01, 0x01];
}

impl Lc3Tools<'_> {
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

        let _ = file.take(str_len as u64).read_to_string(&mut line)?;

        Ok(MemEntry::new(word, orig, line))
    }
}

impl<'a> ObjFileFormat for Lc3Tools<'a> {
    type Parsed = Vec<Loadable>;
    type Return = Lc3ToolsObjFile<'a>;
    const NAME: &'static str = "an LC3Tools Object File";

    fn file_matches_format(file: &mut File) -> bool {
        let mut header_buffer: [u8; 5] = [0; 5];
        if file.read_exact(&mut header_buffer).is_ok() {
            header_buffer == Lc3Tools::HEADER
        } else {
            false
        }
    }

    fn parse(file: &mut File) -> IoResult<Self::Return> {
        if !Self::file_matches_format(file) {
            panic!("Incorrect header.");
        }

        let mut version: [u8; 2] = [0; 2];
        file.read_exact(&mut version)?;

        if version != Lc3Tools::TESTED_VERSION {
            eprintln!(
                "Warning! Untested object file version. ({:?})",
                Lc3Tools::TESTED_VERSION
            );
        }

        let mut memory_entries = Vec::<MemEntry>::new();

        while let Ok(mem_entry) = Lc3Tools::read_mem_entry(file) {
            memory_entries.push(mem_entry);
        }

        Ok(Lc3ToolsObjFile {
            version,
            memory_entries,
            _p: PhantomData,
        })
    }
}
