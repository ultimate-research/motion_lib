mod asm;
mod disasm;
pub mod hash40;
pub mod mlist;

use mlist::MList;
use std::fs::read;
use std::io::{Cursor, Error};

pub fn open(file: &str) -> Result<MList, Error> {
    match read(file) {
        Ok(x) => disasm::disassemble(&mut Cursor::new(x)),
        Err(y) => Err(y),
    }
}
