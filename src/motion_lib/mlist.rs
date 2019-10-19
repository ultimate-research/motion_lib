use crate::hash40::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};

// "motion"
pub const MAGIC: Hash40 = Hash40 {
    value: 0x06f5fea1e8,
};

//TODO: overuse of public attributes? Create .new method instead?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MList {
    pub id_hash: Hash40,
    pub list: IndexMap<Hash40, Motion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motion {
    pub game_script: Hash40,
    #[serde(serialize_with = "ser_flags")]
    pub flags: u16,
    pub transition: u8,
    pub animations: Vec<Animation>,
    pub scripts: Vec<Hash40>,
    pub extra: Option<Extra>,
}

fn ser_flags<S: Serializer>(x: &u16, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&format!("0x{:0x}", x))
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
