extern crate rlua;

use self::rlua::{Function, Lua, Result};

#[allow(dead_code)]
pub fn start_lua() -> Result<()> {
    let lua = Lua::new();
    let lua_globals = lua.globals();
    lua_globals.set("startup_string", "hello world from lua")?;
    let print: Function = lua_globals.get("print")?;
    print.call::<_, ()>("Hello World From Lua")?;
    Ok(())
}
