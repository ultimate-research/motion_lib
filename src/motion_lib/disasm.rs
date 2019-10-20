use crate::hash40::*;
use crate::mlist::*;
use byteorder::{LittleEndian, ReadBytesExt};
use indexmap::IndexMap;
use std::io::{Cursor, Error, ErrorKind};

pub fn disassemble(cursor: &mut Cursor<Vec<u8>>) -> Result<MList, Error> {
    cursor.set_position(0);
    if MAGIC != cursor.read_hash40::<LittleEndian>()? {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "File header is invalid (is this a motion_list.bin file?)",
        ));
    }
    let id = cursor.read_hash40::<LittleEndian>()?;
    let count = cursor.read_u64::<LittleEndian>()?;
    let mut motion_list = IndexMap::<Hash40, Motion>::with_capacity(count as usize);

    for _ in 0..count {
        let motion_kind = cursor.read_hash40::<LittleEndian>()?;
        let motion = read_motion(cursor)?;
        motion_list.insert(motion_kind, motion);
    }

    Ok(MList {
        id_hash: id,
        list: motion_list,
    })
}

fn read_motion(cursor: &mut Cursor<Vec<u8>>) -> Result<Motion, Error> {
    let game = cursor.read_hash40::<LittleEndian>()?;
    let flags = cursor.read_u16::<LittleEndian>()?;
    let frames = cursor.read_u8()?;
    let anm_cnt = cursor.read_u8()?;
    if anm_cnt > 3 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Animation count cannot exceed 3",
        ));
    }
    let size = cursor.read_u32::<LittleEndian>()?;

    let mut anims = Vec::<Animation>::with_capacity(anm_cnt as usize);
    for _ in 0..anm_cnt {
        anims.push(Animation {
            name: cursor.read_hash40::<LittleEndian>()?,
            unk: 0,
        });
    }
    for i in 0..anm_cnt {
        anims[i as usize].unk = cursor.read_u8()?
    }

    //align by 4
    cursor.set_position((cursor.position() + 3 >> 2) << 2);

    let count = size / 8;
    let mut scripts = Vec::<Hash40>::with_capacity(count as usize);
    for _ in 0..count {
        scripts.push(cursor.read_hash40::<LittleEndian>()?);
    }

    let extra: Option<Extra> = if size % 8 == 4 {
        Some(Extra {
            xlu_start: cursor.read_u8()?,
            xlu_end: cursor.read_u8()?,
            cancel_frame: cursor.read_u8()?,
            no_stop_intp: cursor.read_u8()? > 0,
        })
    } else {
        None
    };

    Ok(Motion {
        game_script: game,
        flags: flags,
        transition: frames,
        animations: anims,
        scripts: scripts,
        extra: extra,
    })
}
