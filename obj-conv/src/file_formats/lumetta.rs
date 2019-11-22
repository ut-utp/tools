/// [`ObjFileFormat`](super::ObjFileFormat) implementation for the file format
/// used by [Steven S. Lumetta's assembler and simulator]
/// (http://highered.mheducation.com/sites/0072467509/student_view0/lc-3_simulator.html).
use super::{IoResult, Loadable, ObjFileFormat};

use lc3_isa::Addr;

use std::convert::TryInto;
use std::fmt::{self, Display};
use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LumettaObjFile {
    pairs: Vec<Loadable>,
}

impl Display for LumettaObjFile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.pairs
            .iter()
            .try_for_each(|(addr, word)| writeln!(fmt, "<{:04X}>: {:04X}", addr, word))
    }
}

impl From<LumettaObjFile> for Vec<Loadable> {
    fn from(obj: LumettaObjFile) -> Self {
        obj.pairs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lumetta;

impl ObjFileFormat for Lumetta {
    type Parsed = LumettaObjFile;
    const NAME: &'static str = "an Object File for Steven S. Lumetta's simulator and assembler";

    fn file_matches_format(file: &mut File) -> bool {
        if let Ok(m) = file.metadata() {
            let len = m.len();

            // At least 1 instruction and an even number of bytes:
            len >= 4 && (len % 2) == 0
        } else {
            false
        }
    }

    fn parse(file: &mut File) -> IoResult<Self::Parsed> {
        if !Self::file_matches_format(file) {
            panic!("Invalid object file.");
        }

        let mut pairs: Vec<Loadable> =
            Vec::with_capacity(((file.metadata()?.len() / 2) - 1).try_into().unwrap());
        let mut addr: Addr = file.read_u16::<LittleEndian>()?;

        while let Ok(word) = file.read_u16::<LittleEndian>() {
            pairs.push((addr, word));
            addr += 1;
        }

        Ok(LumettaObjFile { pairs })
    }
}
