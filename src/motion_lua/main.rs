use motion_lib;
use rlua::{Lua, Result, UserData, UserDataMethods, MetaMethod, Value, MultiValue, ToLua, Error};
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

#[derive(Debug, Clone)]
struct MListGlobal {
    //inner: motion_lib::mlist::MList,
    open_root: String,
    save_root: String,
}
#[derive(Debug, Clone)]
struct MListLua {
    inner: motion_lib::mlist::MList,
}
#[derive(Debug, Clone)]
struct MotionLua {
    inner: motion_lib::mlist::Motion,
}

impl UserData for MListGlobal {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |ctx, this, key: String| {
            match key.as_ref() {
                "open_root" => {
                    this.open_root
                        .clone()
                        .to_lua(ctx)
                }
                "save_root" => {
                    this.save_root
                        .clone()
                        .to_lua(ctx)
                }
                _ => Ok(Value::Nil),
            }
        });

        methods.add_meta_method_mut(MetaMethod::NewIndex, |_ctx, this, (key, value): (String, Value)| {
            match key.as_ref() {
                "open_root" => {
                    if let Value::String(s) = value {
                        this.open_root = String::from(s.to_str()?);
                        Ok(())
                    } else {
                        Err(Error::RuntimeError(
                            format!("Expected string value for field {}", &key)
                        ))
                    }
                }
                "save_root" => {
                    if let Value::String(s) = value {
                        this.save_root = String::from(s.to_str()?);
                        Ok(())
                    } else {
                        Err(Error::RuntimeError(
                            format!("Expected string value for field {}", &key)
                        ))
                    }
                }
                _ => {
                    Err(Error::RuntimeError(
                        format!("Key does not exist: {}", &key)
                    ))
                }
            }
        })
    }
}

fn do_lua(code: &str) -> Result<()> {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        let g = lua_ctx.globals();
        g.set("MList", MListGlobal{
            open_root: String::default(),
            save_root: String::default(),
        })?;

        lua_ctx.load(code).exec()
    })
}