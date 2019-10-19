use motion_lib::hash40;
use rlua::{Lua, Result};
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

mod userdata;

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
            if let Err(e) = hash40::load_labels(&args[2]) {
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
        g.set(
            "Lib",
            userdata::LibGlobal {
                open_root: PathBuf::default(),
                save_root: PathBuf::default(),
            },
        )?;
        g.set(
            "hash",
            lua_ctx.create_function(|_ctx, s: String| Ok(hash40::to_hash40(&s).value))?,
        )?;
        g.set(
            "label",
            lua_ctx.create_function(|_ctx, n: i64| {
                Ok(hash40::Hash40 { value: n as u64 }.to_label())
            })?,
        )?;

        lua_ctx.load(code).exec()
    })
}
