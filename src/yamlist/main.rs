use motion_lib;
use serde_yaml::{from_str, to_string};
use std::fs::File;
use std::io::prelude::*;

mod args;
use args::{Args, Mode};
use structopt::StructOpt;

mod error;
use error::ErrorMessage;

type Result<T> = std::result::Result<T, ErrorMessage>;

fn main() {
    let args = Args::from_args();

    if let Some(ref label_path) = args.get_label() {
        if let Err(e) = motion_lib::hash40::load_labels(label_path) {
            println!("Error loading labels: {}", e);
        }
    }

    if let Err(y) = match &args.mode {
        Mode::Disasm {file, ..} => {
            convert_to_yaml(&file, &args.get_outfile())
        }
        Mode::Asm {file, ..} => {
            convert_to_bin(&file, &args.get_outfile())
        }
        Mode::Patch {..} => {
            unimplemented!()
        }
        Mode::Diff {..} => {
            unimplemented!()
        }
    } {
        println!("ERROR: {}", y);
    }
}

fn convert_to_yaml(in_path: &str, out_path: &str) -> Result<()> {
    let x = motion_lib::open(in_path)?;
    let mut f = File::create(out_path)?;
    let pretty = to_string(&x)?;
    f.write_all(pretty.as_bytes())?;
    Ok(())
}

fn convert_to_bin(in_path: &str, out_path: &str) -> Result<()> {
    let mut file = File::open(in_path)?;
    let mut contents: String = String::default();
    file.read_to_string(&mut contents)?;
    
    let mlist = from_str(&contents)?;
    motion_lib::save(out_path, &mlist)?;
    Ok(())
}
