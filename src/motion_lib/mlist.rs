use diff::Diff;
use hash40::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const MAGIC: Hash40 = hash40!("motion");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MList {
    pub motion_path: Hash40,
    pub list: IndexMap<Hash40, Motion>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Diff)]
#[diff(attr(#[derive(Debug, Serialize, Deserialize)]))]
pub struct Motion {
    pub game_script: Hash40,
    pub flags: Flags,
    pub blend_frames: u8,
    pub animations: Vec<Animation>,
    pub scripts: Vec<Hash40>,
    pub extra: Option<Extra>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize, Deserialize)]))]
pub struct Animation {
    pub name: Hash40,
    pub unk: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize, Deserialize)]))]
pub struct Extra {
    pub xlu_start: u8,
    pub xlu_end: u8,
    pub cancel_frame: u8,
    pub no_stop_intp: bool,
}

macro_rules! make_flags {
    ($first:ident, $($names:ident),*) => {
        #[derive(Debug, Default, PartialEq, Copy, Clone, Serialize, Deserialize, Diff)]
        #[diff(attr(#[derive(Debug, Serialize, Deserialize)]))]
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
    turn, r#loop, r#move, fix_trans, fix_rot, fix_scale, unk_40, unk_80, unk_100, unk_200, unk_400
);

#[derive(Debug, Serialize, Deserialize)]
pub struct MListDiff {
    motion_path: <Hash40 as Diff>::Repr,
    altered: IndexMap<Hash40, <Motion as Diff>::Repr>,
    removed: HashSet<Hash40>,
}

impl Diff for MList {
    type Repr = MListDiff;

    fn diff(&self, other: &Self) -> Self::Repr {
        let mut diff = MListDiff {
            motion_path: None,
            altered: IndexMap::new(),
            removed: HashSet::new(),
        };
        if self.motion_path != other.motion_path {
            diff.motion_path = Some(other.motion_path);
        }
        for (key, value) in &self.list {
            if let Some(other_value) = other.list.get(key) {
                if value != other_value {
                    diff.altered.insert(key.clone(), value.diff(other_value));
                }
            } else {
                diff.removed.insert(key.clone());
            }
        }
        for (key, value) in &other.list {
            if let None = self.list.get(key) {
                diff.altered
                    .insert(key.clone(), Motion::identity().diff(&value));
            }
        }
        diff
    }

    fn apply(&mut self, diff: &Self::Repr) {
        if let Some(path_diff) = diff.motion_path {
            self.motion_path = path_diff;
        }
        diff.removed.iter().for_each(|del| {
            self.list.swap_remove(del);
        });
        for (key, change) in &diff.altered {
            if let Some(original) = self.list.get_mut(key) {
                original.apply(change);
            } else {
                self.list
                    .insert(key.clone(), Motion::identity().apply_new(change));
            }
        }
    }

    fn identity() -> Self {
        Default::default()
    }
}
