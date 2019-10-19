use motion_lib;
use rlua::{Error, MetaMethod, MultiValue, ToLua, ToLuaMulti, UserData, UserDataMethods, Value};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct LibGlobal {
    pub open_root: PathBuf,
    pub save_root: PathBuf,
}
#[derive(Debug, Clone)]
pub(crate) struct MListLua<'ml> {
    pub save_root: PathBuf, //copied from Lib when created
    pub rel_path: PathBuf,  //copied, but may be overriden in the save method
    pub inner: &'ml motion_lib::mlist::MList,
}
#[derive(Debug, Clone)]
pub(crate) struct MotionLua<'m> {
    pub inner: &'m motion_lib::mlist::Motion,
}

impl UserData for LibGlobal {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("open", |ctx, this, path: String| {
            let rel = PathBuf::from(path);
            let full_open_path = this.open_root.join(rel.clone());

            match motion_lib::open(&full_open_path) {
                Ok(x) => MListLua {
                    save_root: this.save_root.clone(),
                    rel_path: rel,
                    inner: &x,
                }
                .to_lua_multi(ctx),
                Err(y) => Ok(MultiValue::from_vec(vec![
                    Value::Nil,
                    format!("{}", y).to_lua(ctx)?,
                ])),
            }
        });

        //__index
        methods.add_meta_method(MetaMethod::Index, |ctx, this, key: String| {
            match key.as_ref() {
                "open_root" => this.open_root.to_str().to_lua(ctx),
                "save_root" => this.save_root.to_str().to_lua(ctx),
                _ => Ok(Value::Nil),
            }
        });

        //__newindex
        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |_ctx, this, (key, value): (String, Value)| match key.as_ref() {
                "open_root" => {
                    if let Value::String(s) = value {
                        this.open_root = PathBuf::from(s.to_str()?);
                        Ok(())
                    } else {
                        Err(Error::RuntimeError(format!(
                            "Expected string value for field {}",
                            &key
                        )))
                    }
                }
                "save_root" => {
                    if let Value::String(s) = value {
                        this.save_root = PathBuf::from(s.to_str()?);
                        Ok(())
                    } else {
                        Err(Error::RuntimeError(format!(
                            "Expected string value for field {}",
                            &key
                        )))
                    }
                }
                _ => Err(Error::RuntimeError(format!("Key does not exist: {}", &key))),
            },
        )
    }
}

impl<'a> UserData for MListLua<'a> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("save", |ctx, this, path: Option<String>| {
            let full = match path {
                Some(specified) => this.save_root.join(PathBuf::from(specified)),
                None => this.save_root.join(this.rel_path.clone()),
            };
            match motion_lib::save(full, &this.inner) {
                Ok(_) => Ok(MultiValue::default()),
                Err(y) => Ok(MultiValue::from_vec(vec![
                    Value::Nil,
                    format!("{}", y).to_lua(ctx)?,
                ])),
            }
        });

        methods.add_method("to_table", |ctx, this, ()| {
            let t = ctx.create_table()?;
            let hashes = ctx.create_table()?;
            let motions = ctx.create_table()?;

            let mut index = 1;
            for (hash, motion) in this.inner.list.iter() {
                hashes.set(index, hash.value)?;
                motions.set(hash.value, MotionLua { inner: &motion })?;
                index += 1;
            }
            t.set("hashes", hashes)?;
            t.set("motions", motions)?;
            Ok(Value::Table(t))
        });

        methods.add_method("by_hash", |_, this, hash: i64| {
            let h = motion_lib::hash40::Hash40 { value: hash as u64 };
            Ok(MotionLua {
                inner: &this.inner.list[&h],
            })
        });

        methods.add_method("by_name", |_, this, name: String| {
            let h = motion_lib::hash40::to_hash40(&name);
            Ok(MotionLua {
                inner: &this.inner.list[&h],
            })
        });

        methods.add_method("iter", |ctx, this, ()| {
            let mut state = this.inner.list.iter_mut();
            ctx.create_function(|ctx2, ()| {
                let rets = Vec::<Value>::with_capacity(2);
                match state.next() {
                    Some((hash, motion)) => {
                        rets.push(Value::Integer(hash.value as i64));
                        rets.push(MotionLua { inner: &motion }.to_lua(ctx2)?);
                    }
                    None => {
                        rets.push(Value::Nil);
                        rets.push(Value::Nil);
                    }
                };
                Ok(MultiValue::from_vec(rets))
            })
        });

        methods.add_meta_method(MetaMethod::Index, |_ctx, this, key: String| {
            match key.as_ref() {
                "id" => Ok(Value::Integer(this.inner.id_hash.value as i64)),
                _ => Ok(Value::Nil),
            }
        });
    }
}

impl<'m> UserData for MotionLua<'m> {}
