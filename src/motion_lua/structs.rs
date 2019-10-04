use motion_lib;
use rlua::{Lua, Result, UserData, UserDataMethods, MetaMethod, Value, MultiValue, ToLua, ToLuaMulti, Error};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct LibGlobal {
    pub open_root: PathBuf,
    pub save_root: PathBuf,
}
#[derive(Debug, Clone)]
pub(crate) struct MListLua {
    pub save_to: PathBuf,
    pub inner: motion_lib::mlist::MList,
}
#[derive(Debug, Clone)]
pub(crate) struct MotionLua {
    pub inner: motion_lib::mlist::Motion,
}

impl UserData for LibGlobal {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("open", |ctx, this, path: String| {
            let rel = PathBuf::from(path);
            let full_open_path = this.open_root.join(rel.clone());
            let full_save_path = this.save_root.join(rel);

            match motion_lib::open(&full_open_path) {
                Ok(x) => {
                    MListLua {
                        save_to: full_save_path,
                        inner: x,
                    }.to_lua_multi(ctx)
                }
                Err(y) => {
                    Ok(
                        MultiValue::from_vec(vec!(
                            Value::Nil,
                            format!("{}", y).to_lua(ctx)?,
                        ))
                    )
                }
            }
        });

        //__index
        methods.add_meta_method(MetaMethod::Index, |ctx, this, key: String| {
            match key.as_ref() {
                "open_root" => {
                    this.open_root
                        .to_str()
                        .to_lua(ctx)
                }
                "save_root" => {
                    this.save_root
                        .to_str()
                        .to_lua(ctx)
                }
                _ => Ok(Value::Nil),
            }
        });

        //__newindex
        methods.add_meta_method_mut(MetaMethod::NewIndex, |_ctx, this, (key, value): (String, Value)| {
            match key.as_ref() {
                "open_root" => {
                    if let Value::String(s) = value {
                        this.open_root = PathBuf::from(s.to_str()?);
                        Ok(())
                    } else {
                        Err(Error::RuntimeError(
                            format!("Expected string value for field {}", &key)
                        ))
                    }
                }
                "save_root" => {
                    if let Value::String(s) = value {
                        this.save_root = PathBuf::from(s.to_str()?);
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

impl UserData for MListLua {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        //placeholder
    }
}