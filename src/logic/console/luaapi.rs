use anyhow::Result;
use rglua::{cstr, lua::{LuaState, REGISTRYINDEX, lua_pushvalue, luaL_checkstring, luaL_ref}, lua_function};
use crate::{call_interface_function, engine::interfaces::client::INTERFACE_ENGINE_CLIENT, logic::console::detour};

#[lua_function]
pub fn api_execute_command(lua: LuaState) -> Result<i32> {
  let command = luaL_checkstring(lua, 1);

  call_interface_function!(INTERFACE_ENGINE_CLIENT, ClientCmd, command);

  Ok(0)
}

#[lua_function]
pub fn api_detour_log(_lua: LuaState) -> Result<i32> {
  detour::logging::detour()?;
  Ok(0)
}

#[lua_function]
pub fn api_undetour_log(_lua: LuaState) -> Result<i32> {
  detour::logging::undetour()?;
  Ok(0)
}

#[lua_function]
pub fn api_detour_concommands(_lua: LuaState) -> Result<i32> {
  detour::commands::detour()?;

  Ok(0)
}

#[lua_function]
pub fn api_hook_concommand(lua: LuaState) -> Result<i32> {
  let command_name = luaL_checkstring(lua, 2);
  lua_pushvalue(lua, 1);
  let callback = luaL_ref(lua, REGISTRYINDEX);

  detour::commands::hook_command(lua, command_name, callback);

  Ok(0)
}

#[lua_function]
pub fn api_undetour_concommands(_lua: LuaState) -> Result<i32> {
  detour::commands::undetour();
  Ok(0)
}