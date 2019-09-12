mod asm;
mod disasm;
pub mod hash40;
pub mod mlist;

use mlist::*;
use hash40::*;
use std::io::Error;

pub fn open(file: &str) -> Result<MList, String> {
    
    Err(String::from("not implemented here"))
}