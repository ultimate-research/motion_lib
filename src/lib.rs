mod asm;
mod disasm;
#[allow(clippy::all)]
pub mod mlist;

use mlist::MList;
use std::fs::{read, File};
use std::io::{prelude::*, Cursor, Error};
use std::path::Path;

pub use diff;
pub use hash40;

/// Attempts to read a [MList] from the given reader (requires [Seek]).
/// The reader should be positioned at the start of the filetype.
/// Returns a [MList] if successful, otherwise an [Error].
pub fn read_stream<R: Read + Seek>(reader: &mut R) -> Result<MList, Error> {
    disasm::disassemble(reader)
}

/// Attempts to write a [MList] into the given writer (requires [Seek]).
/// Returns nothing if successful, otherwise an [Error].
pub fn write_stream<W: Write + Seek>(writer: &mut W, mlist: &MList) -> Result<(), Error> {
    asm::assemble(writer, mlist)
}

/// Attempts to read a [MList] from a file path
pub fn open<P: AsRef<Path>>(file: P) -> Result<MList, Error> {
    disasm::disassemble(&mut Cursor::new(read(file)?))
}

/// Attempts to write a [MList] to a file path
pub fn save<P: AsRef<Path>>(path: P, mlist: &MList) -> Result<(), Error> {
    let mut file = File::create(path)?;
    let mut cursor = Cursor::new(Vec::<u8>::new());
    asm::assemble(&mut cursor, mlist)?;
    file.write_all(&cursor.into_inner())?;
    Ok(())
}
