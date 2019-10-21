mod asm;
mod disasm;
#[allow(clippy::all)]
pub mod hash40;
pub mod mlist;

#[macro_use]
extern crate lazy_static;

use mlist::MList;
use std::fs::{read, File};
use std::io::{prelude::*, Cursor, Error};
use std::path::Path;

pub fn open<P: AsRef<Path>>(file: P) -> Result<MList, Error> {
    match read(file) {
        Ok(x) => disasm::disassemble(&mut Cursor::new(x)),
        Err(y) => Err(y),
    }
}

pub fn save<P: AsRef<Path>>(path: P, mlist: &MList) -> Result<(), Error> {
    match File::create(path) {
        Ok(mut file) => {
            let mut cursor = Cursor::new(Vec::<u8>::new());
            asm::assemble(&mut cursor, mlist)?;
            file.write_all(&cursor.into_inner())?;
            Ok(())
        }
        Err(y) => Err(y),
    }
}
