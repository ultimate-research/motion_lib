use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub mode: Mode,
    
    #[structopt(long, short)]
    pub label: Option<String>,

    #[structopt(long, short)]
    pub out: Option<String>,
}

#[derive(StructOpt)]
pub enum Mode {
    #[structopt(about = "Convert from motion_list to yaml")]
    Disasm {
        file: String,
    },

    #[structopt(about = "Convert from yaml to motion_list")]
    Asm {
        file: String,
    },

    Patch {
        file: String,
        patch: String,
    },

    Diff {
        a: String,
        b: String,
    },
}
