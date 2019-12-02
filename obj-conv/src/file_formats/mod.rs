/// Object File Formats and a trait.
use super::Loadable;

use lc3_isa::util::LoadableIterator;

use std::fmt::Display;
use std::fs::File;
// use std::ops::Deref;
pub(crate) use super::IoResult;

pub trait ObjFileFormat {
    type Parsed: LoadableIterator;
    // type Return: Deref<Target = Self::Parsed>;
    type Return: Into<Self::Parsed> + Display + Clone;
    const NAME: &'static str;

    /// Returns true if the file can be interpreted as the object file format
    /// that the implementor represents.
    ///
    /// Implementations of this function are allowed to seek the file; it will
    /// be discarded and a fresh file will be opened and passed to the parse
    /// function.
    fn file_matches_format(file: &mut File) -> bool;

    /// Parses the file into the object file format's parsed type.
    fn parse(file: &mut File) -> IoResult<Self::Return>;
}

pub mod lc3tools;
pub mod lumetta;

pub use lc3tools::Lc3Tools;
pub use lumetta::Lumetta;

