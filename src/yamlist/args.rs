use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub mode: Mode,

    #[structopt(long, short, global(true))]
    pub label: Option<String>,

    #[structopt(long, short, global(true))]
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

    #[structopt(about = "Take two motion_lists, and produce a yaml file of their difference")]
    Diff {
        a: String,
        b: String,
    },
}
