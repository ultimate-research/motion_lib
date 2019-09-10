use crate::hash40;

pub const MAGIC: u64 = hash40::calc_hash40("motion");

#[derive(Debug)]
pub struct MList {
    id_hash: u64,
    list: Vec<Motion>,
}

#[derive(Debug)]
pub struct Motion {
    kind: u64,
    game_script: u64,
    flags: u16,
    transition: u8,
    animations: Vec<Animation>,
    scripts: 
}

#[derive(Debug)]
pub struct Animation {
    name: u64,
    unk: u8,
}

#[derive(Debug)]
pub enum ScriptKind {
    Expression,
    Sound,
    Effect,
    Game2,
    Expression2,
    Sound2,
    Effect2
}

#[derive(Debug)]
pub enum ScriptGroup {
    None     = 0,
    F        = 1,
    SF       = 2,
    XSF      = 3,
    SFG2S2F2 = 5,
}