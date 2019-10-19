use motion_lib;
use serde_yaml::{from_str, to_string};
use std::borrow::Borrow;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    let mut mode = 0;
    let mut filename = String::default();
    let mut labelname = String::default();
    let mut outname = String::default();

    if len <= 1 {
        print_help_text();
        return;
    }

    let mut i = 1;
    while i < len {
        match args[i].as_ref() {
            "-h" => print_help_text(),
            "-d" => mode = 1,
            "-a" => mode = 2,
            "-l" => {
                i += 1;
                if i < len {
                    labelname = String::from(&args[i]);
                } else {
                    println!("missing 'file' arg for label");
                }
            }
            "-o" => {
                i += 1;
                if i < len {
                    outname = String::from(&args[i]);
                } else {
                    println!("missing 'file' arg for output name");
                }
            }
            _ => filename = String::from(&args[i]),
        }
        i += 1;
    }
    if filename.len() == 0 {
        println!("missing 'file' in args");
    } else if mode == 0 {
        println!("missing '-d' or '-a' mode in args");
    } else if mode == 1 {
        if labelname.len() > 0 {
            if let Err(e) = motion_lib::hash40::load_labels(&labelname) {
                println!("Error loading labels: {}", e);
            }
        }

        let o = if outname.len() > 0 {
            &outname
        } else {
            "out.yml"
        };

        match convert_to_yaml(&filename, o) {
            Ok(_) => {}
            Err(y) => {
                let e: &Error = y.borrow();
                println!("ERROR: {}", e);
            }
        }
    } else if mode == 2 {
        if labelname.len() > 0 {
            if let Err(e) = motion_lib::hash40::load_labels(&labelname) {
                println!("Error loading labels: {}", e);
            }
        }

        let o = if outname.len() > 0 {
            &outname
        } else {
            "out.bin"
        };

        match convert_to_bin(&filename, o) {
            Ok(_) => {}
            Err(y) => {
                let e: &Error = y.borrow();
                println!("ERROR: {}", e);
            }
        }
    }
}

fn print_help_text() {
    println!("Args: [file] [-d or -a] [other]");
    println!("  -h (help text)    = print this help text");
    println!("  -d (disassemble)  = convert to readable YAML");
    println!("  -a (assemble)     = convert to binary");
    println!("  -l (label) <file> = load labels");
    println!("  -o (out)   <file> = set output name");
}

fn convert_to_yaml(i: &str, o: &str) -> Result<(), Box<Error>> {
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

fn convert_to_bin(i: &str, o: &str) -> Result<(), Box<Error>> {
    let mut f = File::open(i)?;
    let mut s: String = String::default();
    f.read_to_string(&mut s)?;
    match from_str::<motion_lib::mlist::MList>(&s) {
        Ok(x) => match motion_lib::save(o, &x) {
            Ok(_) => Ok(()),
            Err(y) => Err(Box::new(y)),
        },
        Err(y) => Err(Box::new(y)),
    }
}
