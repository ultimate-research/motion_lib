use diff::Diff;
use hash40::{read_labels, set_labels};
use serde_yaml::{from_str, to_string};
use std::fs::File;
use std::io::prelude::*;

mod args;
use args::{Args, Mode};
use structopt::StructOpt;

mod error;
use error::{ErrorMessage, ErrorString};

type Result<T> = std::result::Result<T, ErrorMessage>;

fn main() {
    let args = Args::from_args();

    if let Some(ref label_path) = args.label {
        match read_labels(label_path) {
            Ok(labels) => set_labels(labels),
            Err(e) => println!("Error loading labels: {}", e),
        }
    }

    if let Err(y) = match &args.mode {
        Mode::Disasm { file, .. } => {
            convert_to_yaml(&file, &args.out.as_ref().map_or("out.yml", String::as_str))
        }
        Mode::Asm { file, .. } => {
            convert_to_bin(&file, &args.out.as_ref().map_or("out.bin", String::as_str))
        }
        Mode::Patch { .. } => patch_motion_bin(),
        Mode::Diff { a, b } => {
            diff_files(a, b, &args.out.as_ref().map_or("diff.yml", String::as_str))
        }
    } {
        println!("ERROR: {}", y);
    }
}

// TODO: args/implementation
fn patch_motion_bin() -> Result<()> {
    Err(ErrorString("Patching not supported").into())
}

// TODO: args/implementation
fn diff_files(a: &str, b: &str, out_path: &str) -> Result<()> {
    let a = motion_lib::open(a)?;
    let b = motion_lib::open(b)?;
    let diff = a.diff(&b);
    let mut f = File::create(out_path)?;
    let pretty = to_string(&diff)?;
    f.write_all(pretty.as_bytes())?;
    Ok(())
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
