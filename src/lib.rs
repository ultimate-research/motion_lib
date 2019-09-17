mod asm;
mod disasm;
pub mod hash40;
pub mod mlist;

#[macro_use]
extern crate lazy_static;

use mlist::MList;
use std::io::{Cursor, Error, prelude::*};
use std::fs::{read, File};

pub fn open(file: &str) -> Result<MList, Error> {
    match read(file) {
        Ok(x) => disasm::disassemble(&mut Cursor::new(x)),
        Err(y) => Err(y),
    }
}

pub fn save(file: &str, mlist: &MList) -> Result<(), Error> {
    match File::create(file) {
        Ok(mut x) => {
            let mut cursor = Cursor::new(Vec::<u8>::new());
            asm::assemble(&mut cursor, mlist)?;
            x.write_all(&cursor.into_inner())?;
            Ok(())
        }
        Err(y) => Err(y),
    }
}
