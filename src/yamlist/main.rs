use motion_lib;
use serde_yaml::{from_str, to_string};
use std::borrow::Borrow;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod args;
use args::{Args, Mode};
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    if let Some(labelname) = args.label {
        if let Err(e) = motion_lib::hash40::load_labels(&labelname) {
            println!("Error loading labels: {}", e);
        }
    }

    match args.mode {
        Mode::Disasm {file} => {
            let o = args.out.unwrap_or("out.yml".into());

            match convert_to_yaml(&file, &o) {
                Ok(_) => {}
                Err(y) => {
                    let e: &dyn Error = y.borrow();
                    println!("ERROR: {}", e);
                }
            }
        }
        Mode::Asm {file} => {
            let o = args.out.unwrap_or("out.yml".into());

            match convert_to_bin(&file, &o) {
                Ok(_) => {}
                Err(y) => {
                    let e: &dyn Error = y.borrow();
                    println!("ERROR: {}", e);
                }
            }
        }
        Mode::Patch {..} => {
            unimplemented!()
        }
        Mode::Diff {..} => {
            unimplemented!()
        }
    }
}

fn convert_to_yaml(i: &str, o: &str) -> Result<(), Box<dyn Error>> {
    match motion_lib::open(i) {
        Ok(x) => {
            let mut f = File::create(o)?;
            let pretty = to_string(&x)?;
            f.write_all(pretty.as_bytes())?;
            Ok(())
        }
        Err(y) => Err(Box::new(y)),
    }
}

fn convert_to_bin(in_path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(in_path)?;
    let mut contents: String = String::default();
    file.read_to_string(&mut contents)?;
    match from_str(&contents) {
        Ok(mlist) => match motion_lib::save(out_path, &mlist) {
            Ok(_) => Ok(()),
            Err(y) => Err(Box::new(y)),
        },
        Err(y) => Err(Box::new(y)),
    }
}
