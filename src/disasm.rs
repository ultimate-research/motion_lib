use crate::mlist::*;
use byteorder::{LittleEndian, ReadBytesExt};
use hash40::*;
use indexmap::IndexMap;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

pub fn disassemble<R: Read + Seek>(cursor: &mut R) -> Result<MList, Error> {
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
        motion_path: id,
        list: motion_list,
    })
}

fn read_motion<R: Read + Seek>(cursor: &mut R) -> Result<Motion, Error> {
    let game_script = cursor.read_hash40::<LittleEndian>()?;
    let flags = cursor.read_u16::<LittleEndian>()?.into();
    let blend_frames = cursor.read_u8()?;
    let anm_cnt = cursor.read_u8()?;
    if anm_cnt > 3 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Animation count cannot exceed 3",
        ));
    }
    let size = cursor.read_u32::<LittleEndian>()?;

    let animations = (0..anm_cnt)
        .map(|_| cursor.read_hash40::<LittleEndian>())
        .collect::<Result<Vec<_>, Error>>()?
        .iter()
        .map(|name| {
            Ok(Animation {
                name: *name,
                unk: cursor.read_u8()?,
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    //align by 4
    const ALIGN: u64 = 4;
    const ALIGN_MASK: u64 = ALIGN - 1;
    let pos = cursor.seek(SeekFrom::Current(0))?;
    cursor.seek(SeekFrom::Start((pos + ALIGN_MASK) & !ALIGN_MASK))?;

    let scripts = (0..size / 8)
        .map(|_| cursor.read_hash40::<LittleEndian>())
        .collect::<Result<Vec<_>, Error>>()?;

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
        game_script,
        flags,
        blend_frames,
        animations,
        scripts,
        extra,
    })
}
