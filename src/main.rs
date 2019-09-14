use motion_lib;
use serde_yaml::to_string;
use std::env;
use std::fs::File;
use std::io::{Error, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args[1];

    match main_sub(f) {
        Ok(_x) => {}
        Err(y) => {
            println!("ERROR: {}", y);
        }
    }
}

fn main_sub(i: &str) -> Result<(), Error> {
    match motion_lib::open(i) {
        Ok(x) => {
            let mut o = File::create("motion_list.yml")?;
            let pretty = to_string(&x).unwrap();
            o.write_all(pretty.as_bytes())?;
            Ok(())
        }
        Err(y) => Err(y),
    }
}
