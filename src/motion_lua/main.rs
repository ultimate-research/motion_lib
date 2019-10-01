use motion_lib;
use rlua::{Lua, Result};
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    let mut lua_file = String::default();

    if len <= 1 || args[1] == "-h" || args[1] == "--help" {
        println!("Args: [lua source file] [optional label file]");
        return;
    }

    if len >= 2 {
        lua_file = args[1].to_owned();

        if len >= 3 {
            if let Err(e) = motion_lib::hash40::load_labels(&args[2]) {
                println!("Error loading labels: {}", e);
            }
        }
    }

    let f;
    match read_to_string(lua_file) {
        Ok(x) => f = x,
        Err(y) => {
            println!("{}", y);
            return;
        }
    }

    match do_lua(&f) {
        Ok(_) => {}
        Err(y) => {
            println!("{}", y);
            return;
        }
    }
}

fn do_lua(code: &str) -> Result<()> {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        let g = lua_ctx.globals();
        
        
        lua_ctx.load(code).exec()
    })
}