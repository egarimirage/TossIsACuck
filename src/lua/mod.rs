extern crate rlua;


// will eventually need the added options MetaMethod, UserData, UserDataMethods, Variadic but those will jsut be unused imports right now.
use self::rlua::{Function, Lua, Result};


#[allow(dead_code)] // leaving this here for when we aren't using lua.
pub fn start_lua() -> Result<()> {
    let lua = Lua::new();
    let lua_globals = lua.globals();
    let print: Function = lua_globals.get("print")?;
    print.call::<_, ()>("Hello World From Lua")?;
    Ok(())
}
