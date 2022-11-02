use clap::Parser;
use motion_lib::diff::Diff;
use motion_lib::hash40::Hash40;
use serde_yaml::{from_str, to_string};

use std::fs::File;
use std::io::prelude::*;

mod args;
mod error;

use args::{Args, Mode};
use error::ErrorMessage;

type Result<T> = std::result::Result<T, ErrorMessage>;

fn main() {
    let args = Args::parse();

    if let Some(ref label_path) = args.label {
        let label_clone = Hash40::label_map();
        let mut labels = label_clone.lock().unwrap();
        labels.add_labels_from_path(label_path).unwrap();
    }

    if let Err(y) = match &args.mode {
        Mode::Disasm { file, .. } => {
            convert_to_yaml(file, args.out.as_ref().map_or("out.yml", String::as_str))
        }
        Mode::Asm { file, .. } => {
            convert_to_bin(file, args.out.as_ref().map_or("out.bin", String::as_str))
        }
        Mode::Diff { a, b } => {
            diff_files(a, b, args.out.as_ref().map_or("diff.yml", String::as_str))
        }
        Mode::Patch { file, patch } => patch_motion_bin(
            file,
            patch,
            args.out.as_ref().map_or("patched.bin", String::as_str),
        ),
    } {
        println!("ERROR: {}", y);
    }
}

fn patch_motion_bin(file: &str, patch: &str, out_path: &str) -> Result<()> {
    let a = motion_lib::open(file)?;
    let mut contents: String = String::default();
    File::open(patch)?.read_to_string(&mut contents)?;
    let diff = from_str(&contents)?;

    let out = a.apply_new(&diff);
    motion_lib::save(out_path, &out)?;
    Ok(())
}

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
