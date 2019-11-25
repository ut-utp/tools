/// Object File Formats and a trait.
use super::Loadable;
use lc3_shims::memory::{FileBackedMemoryShim, MemoryShim};

use core::mem::size_of;
use std::fmt::Display;
use std::fs::File;
pub(crate) use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

use byteorder::{LittleEndian, WriteBytesExt};

// TODO: Actually figure out the imports
pub type Addr = u16;
/// Maximum possible address value.
pub const ADDR_MAX_VAL: Addr = Addr::max_value();
/// Word type/size for the LC-3.
pub type Word = u16;
/// Maximum possible word value.
pub const WORD_MAX_VAL: Word = Word::max_value();
/// Size of the LC-3 address space in [`Word`](Word)s.
pub const ADDR_SPACE_SIZE_IN_WORDS: usize = (ADDR_MAX_VAL as usize) + 1;
/// Size of the LC-3 address space in bytes.
pub const ADDR_SPACE_SIZE_IN_BYTES: usize = ADDR_SPACE_SIZE_IN_WORDS * size_of::<Word>();

pub trait ObjFileFormat {
    type Parsed: Into<Vec<Loadable>> + Display + Clone;
    const NAME: &'static str;

    /// Returns true if the file can be interpreted as the object file format
    /// that the implementor represents.
    ///
    /// Implementations of this function are allowed to seek the file; it will
    /// be discarded and a fresh file will be opened and passed to the parse
    /// function.
    fn file_matches_format(file: &mut File) -> bool;

    /// Parses the file into the object file format's parsed type.
    fn parse(file: &mut File) -> IoResult<Self::Parsed>;

    fn convert_to_mem_map<P: AsRef<Path>>(parsed: Self::Parsed, path: P, with_os: bool) -> IoResult<()> {
        let pairs: Vec<Loadable> = parsed.into();

        let mut mem: FileBackedMemoryShim;
        if with_os {
            let os_path: String = format!("lc3os.mem");
            mem = FileBackedMemoryShim::from_existing_file(&os_path).unwrap();
            mem.change_file(path);
        } else {
            mem = FileBackedMemoryShim::new(path);
        }

        for &(addr, word) in pairs.iter() {
            mem[addr] = word;
        }

        mem.flush();

        Ok(())
    }
}

pub mod lc3tools;
pub mod lumetta;

pub use lc3tools::Lc3Tools;
pub use lumetta::Lumetta;

