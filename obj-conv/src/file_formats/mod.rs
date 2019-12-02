//! Object File Formats and a trait.
use super::Loadable;

use lc3_isa::util::LoadableIterator;

pub(crate) use super::IoResult;
use std::fmt::Display;
use std::fs::File;

/// Interface for a parseable object file format.
pub trait ObjFileFormat {
    /// Type that Self::Return can be converted into that also can be turned
    /// into an Iterator on Loadables (i.e. (Addr, Word) pairs).
    ///
    /// Put differently, this type must implement
    /// IntoIterator<Item = (Addr, Word)>.
    type Parsed: LoadableIterator;

    /// Type that is returned upon parsing.
    /// Must be printable, clonable, and able to be converted into a type that
    /// can be converted into Iterator on (Addr, Word) pairs.
    type Return: Into<Self::Parsed> + Display + Clone;

    /// Human readable name of the format.
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
