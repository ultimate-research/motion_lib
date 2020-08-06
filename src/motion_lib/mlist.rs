use hash40::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub const MAGIC: Hash40 = hash40!("motion");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MList {
    pub motion_path: Hash40,
    pub list: IndexMap<Hash40, Motion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motion {
    pub game_script: Hash40,
    pub flags: Flags,
    pub transition: u8,
    pub animations: Vec<Animation>,
    pub scripts: Vec<Hash40>,
    pub extra: Option<Extra>,
}

pub enum MotionFlag {
    BlendAfter,
    Loop,
    UpdatePosition,
    FixTranslation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: Hash40,
    pub unk: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extra {
    pub xlu_start: u8,
    pub xlu_end: u8,
    pub cancel_frame: u8,
    pub no_stop_intp: bool,
}

macro_rules! make_flags {
    ($first:ident, $($names:ident),*) => {
        #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
        pub struct Flags {
            pub $first: bool,
            $(pub $names: bool),*
        }

        impl From<u16> for Flags {
            fn from(f: u16) -> Flags {
                let mut mask = 1;
                let $first = (f & mask) != 0;
                $(
                    mask <<= 1;
                    let $names = (f & mask) != 0;
                )*
                Flags { $first, $($names),* }
            }
        }

        impl From<Flags> for u16 {
            fn from(f: Flags) -> u16 {
                let mut mask = 1;
                let mut ret = f.$first as u16;
                $(
                    mask <<= 1;
                    if f.$names { ret |= mask }
                )*
                ret
            }
        }
    };
}

make_flags!(
    blend_after,
    r#loop,
    r#move,
    fix_trans,
    fix_rot,
    fix_scale,
    blend_through,
    unk_80,
    unk_100,
    unk_200,
    unk_400
);
