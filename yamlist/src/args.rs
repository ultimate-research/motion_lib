use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub mode: Mode,

    #[arg(long, short, global(true))]
    pub label: Option<String>,

    #[arg(long, short, global(true))]
    pub out: Option<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Mode {
    #[command(about = "Convert from motion_list to yaml")]
    Disasm { file: String },

    #[command(about = "Convert from yaml to motion_list")]
    Asm { file: String },

    #[command(about = "Take two motion_lists, and produce a yaml file of their difference")]
    Diff { a: String, b: String },

    #[command(about = "Take a motion_list and apply a yaml patch to create a new motion_list")]
    Patch { file: String, patch: String },
}
