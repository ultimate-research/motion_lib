use crate::hash40::*;
use crate::mlist::*;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Error, ErrorKind};

pub fn assemble(cursor: &mut Cursor<Vec<u8>>, mlist: &MList) -> Result<(), Error> {
    cursor.write_hash40::<LittleEndian>(&MAGIC)?;
    cursor.write_hash40::<LittleEndian>(&mlist.id_hash)?;
    cursor.write_u64::<LittleEndian>(mlist.list.len() as u64)?;

    for motion in mlist.list.iter() {
        cursor.write_hash40::<LittleEndian>(motion.0)?;
        match write_motion(cursor, motion.1) {
            Ok(_) => {}
            Err(y) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("ERROR in motion kind {}: {}", motion.0.to_label(), y),
                ))
            }
        }
    }

    Ok(())
}

fn write_motion(cursor: &mut Cursor<Vec<u8>>, motion: &Motion) -> Result<(), Error> {
    let anm_cnt = motion.animations.len();
    if anm_cnt > 3 {
        Err(Error::new(
            ErrorKind::InvalidData,
            "Animation count cannot exceed 3",
        ))?;
    }

    cursor.write_hash40::<LittleEndian>(&motion.game_script)?;
    cursor.write_u16::<LittleEndian>(motion.flags)?;
    cursor.write_u8(motion.transition)?;
    cursor.write_u8(anm_cnt as u8)?;

    let temp = (motion.scripts.len() * 8) as u32;
    let size = temp + (motion.extra.is_some() as u32) * 4;
    cursor.write_u32::<LittleEndian>(size)?;

    for i in motion.animations.iter() {
        cursor.write_hash40::<LittleEndian>(&i.name)?;
    }
    for i in motion.animations.iter() {
        cursor.write_u8(i.unk)?;
    }
    //align
    const ALIGN: u64 = 4;
    const ALIGN_MASK: u64 = ALIGN - 1;
    cursor.set_position((cursor.position() + ALIGN_MASK) & !ALIGN_MASK);

    for script in motion.scripts.iter() {
        cursor.write_hash40::<LittleEndian>(&script)?;
    }

    if let Some(x) = &motion.extra {
        cursor.write_u8(x.xlu_start)?;
        cursor.write_u8(x.xlu_end)?;
        cursor.write_u8(x.cancel_frame)?;
        cursor.write_u8(x.no_stop_intp as u8)?;
    }

    Ok(())
}
