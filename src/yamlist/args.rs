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
    
        #[structopt(long, short)]
        label: Option<String>,

        #[structopt(long, short)]
        out: Option<String>,
    },

    #[structopt(about = "Convert from yaml to motion_list")]
    Asm {
        file: String,
    
        #[structopt(long, short)]
        label: Option<String>,

        #[structopt(long, short)]
        out: Option<String>,
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

impl Args {
    pub fn get_label<'a>(&'a self) -> Option<&'a String> {
        if self.label.is_some() {
            self.label.as_ref()
        } else {
            match &self.mode {
                Mode::Asm {label, ..} | Mode::Disasm {label, ..} => {
                    label.as_ref()
                }
                _ => None
            }
        }
    }
    
    pub fn get_outfile<'a>(&'a self) -> &'a str {
        self.out
            .as_ref()
            .map_or(
                match &self.mode {
                    Mode::Asm {out, ..} => {
                        out.as_ref().map_or("out.bin", String::as_str)
                    }
                    Mode::Disasm {out, ..} => {
                        out.as_ref().map_or("out.yml", String::as_str)
                    }
                    _ => "out.bin"
                },
                String::as_str
            )
    }
}
