use crate::mlist::*;
use crate::hash40::*;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io::Cursor;

pub fn disassemble(cursor: &mut Cursor<Vec<u8>>) -> Result<MList, String> {
    cursor.set_position(0);
    assert_eq!(MAGIC, cursor.read_hash40::<LittleEndian>().unwrap());

    Err(String::from("Not implemented here"))
}