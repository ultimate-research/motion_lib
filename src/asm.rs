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
            Err(y) => return Err(Error::new(
                ErrorKind::Other,
                format!("ERROR in motion kind {}: {}", motion.0.to_label(), y)
            ))
        }
    }

    Ok(())
}

fn write_motion(cursor: &mut Cursor<Vec<u8>>, motion: &Motion) -> Result<(), Error> {
    let anm_cnt = motion.animations.len();
    if anm_cnt > 3 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Animation count cannot exceed 3",
        ))
    }

    cursor.write_hash40::<LittleEndian>(&motion.game_script)?;
    cursor.write_u16::<LittleEndian>(motion.flags)?;
    cursor.write_u8(motion.transition)?;
    cursor.write_u8(anm_cnt as u8)?;
    
    let temp = (motion.scripts.len() * 8) as u32;
    let size = temp + (if let Some(_) = &motion.extra { 4 } else { 0 });
    cursor.write_u32::<LittleEndian>(size)?;

    for i in motion.animations.iter() {
        cursor.write_hash40::<LittleEndian>(&i.name)?;
    }
    for i in motion.animations.iter() {
        cursor.write_u8(i.unk)?;
    }
    //align
    cursor.set_position((cursor.position() + 3 >> 2) << 2);

    for script in motion.scripts.iter() {
        cursor.write_hash40::<LittleEndian>(&script)?;
    }

    if let Some(x) = &motion.extra {
        cursor.write_u8(x.xlu_start)?;
        cursor.write_u8(x.xlu_end)?;
        cursor.write_u8(x.cancel_frame)?;
        cursor.write_u8(if x.no_stop_intp {1} else {0})?;
    }

    Ok(())
}