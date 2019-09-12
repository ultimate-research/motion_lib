use crate::mlist::*;
use crate::hash40::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

pub fn disassemble(cursor: &mut Cursor<Vec<u8>>) -> Result<MList, String> {
    cursor.set_position(0);
    assert_eq!(MAGIC, cursor.read_hash40::<LittleEndian>().unwrap());
    let id = cursor.read_hash40::<LittleEndian>().unwrap();
    let motion_list = Vec::<MList>::new();

    Err(String::from("Not implemented here"))
}