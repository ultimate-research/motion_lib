mod asm;
mod disasm;
#[allow(clippy::all)]
pub mod mlist;

use mlist::MList;
use std::fs::{read, File};
use std::io::{prelude::*, Cursor, Error};
use std::path::Path;

pub use hash40;

pub fn open<P: AsRef<Path>>(file: P) -> Result<MList, Error> {
    disasm::disassemble(&mut Cursor::new(read(file)?))
}

pub fn save<P: AsRef<Path>>(path: P, mlist: &MList) -> Result<(), Error> {
    let mut file = File::create(path)?;
    let mut cursor = Cursor::new(Vec::<u8>::new());
    asm::assemble(&mut cursor, mlist)?;
    file.write_all(&cursor.into_inner())?;
    Ok(())
}
