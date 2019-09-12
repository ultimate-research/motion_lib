use crate::hash40::*;
use std::string::ToString;
use std::str::FromStr;

// "motion"
pub const MAGIC: Hash40 = Hash40 {value: 0x06f5fea1e8};

#[derive(Debug)]
pub struct MList {
    id_hash: Hash40,
    list: Vec<Motion>,
}

#[derive(Debug)]
pub struct Motion {
    kind: Hash40,
    game_script: Hash40,
    flags: u16,
    transition: u8,
    animations: Vec<Animation>,
    scripts: Vec<Script>,
    extra: Option<Extra>,
}

#[derive(Debug)]
pub struct Animation {
    name: Hash40,
    unk: u8,
}

#[derive(Debug)]
pub struct Script {
    kind: ScriptKind,
    name: Hash40,
}

#[derive(Debug)]
pub struct Extra {
    xlu_start: u8,
    xlu_end: u8,
    cancel_frame: u8,
    no_stop_intp: u8,
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
enum ScriptGroup {
    None     = 0,
    F        = 1,
    SF       = 2,
    XSF      = 3,
    SFG2S2F2 = 5,
}

//TODO: remove boilerplate
impl ToString for ScriptKind {
    fn to_string(&self) -> String {
        match self {
            ScriptKind::Expression => String::from("expression"),
            ScriptKind::Sound => String::from("sound"),
            ScriptKind::Effect => String::from("effect"),
            ScriptKind::Game2 => String::from("game2"),
            ScriptKind::Expression2 => String::from("expression2"),
            ScriptKind::Sound2 => String::from("sound2"),
            ScriptKind::Effect2 => String::from("effect2"),
        }
    }
}

impl FromStr for ScriptKind {
    type Err = String;

    fn from_str(s: &str) -> Result<ScriptKind, String> {
        match s {
            "expression" => Ok(ScriptKind::Expression),
            "sound" => Ok(ScriptKind::Sound),
            "effect" => Ok(ScriptKind::Effect),
            "game2" => Ok(ScriptKind::Game2),
            "expression2" => Ok(ScriptKind::Expression2),
            "sound2" => Ok(ScriptKind::Sound2),
            "effect2" => Ok(ScriptKind::Effect2),
            _ => Err(format!("Invalid script kind name: {}", s)),
        }
    }
}