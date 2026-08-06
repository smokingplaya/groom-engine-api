use std::cell::OnceCell;
use atomicframework_api::{AtomicFramework, LoggerLevel, Package, PackageKind, PackageMetadata};
use anyhow::Result;
use rglua::{cstr, lua::{LuaState, lua_newtable, lua_pushcfunction, lua_setfield}};
use crate::{logic, events::setup_event_dispatcher, utils::{log, log_with_level, set_logger}};

const GROOM_PACKAGE: PackageMetadata = PackageMetadata {
  id: "dev.smokingplaya.groom.engine-api",
  title: "Groom Source Engine API",
  version: "0.1.0-alpha.1",
  description: Some("This package provides access for low-level functions and events of Source Engine 2013 (gmod branch)."),
  documentation: Some("https://github.com/smokingplaya/groom-engine-api/wiki"),
  homepage_url: Some("https://github.com/smokingplaya/groom-engine-api"),
  kind: PackageKind::System,
  icon: None,
};

thread_local! {
  static PACKAGE_INSTANCE: OnceCell<Package> = const { OnceCell::new() };
}

fn set_package_instance(lua: Package) {
  PACKAGE_INSTANCE.with(|lock| lock.set(lua))
    .unwrap();
}

#[allow(unused)]
pub fn get_package_instance() -> Package {
  PACKAGE_INSTANCE.with(|cell| cell.get().unwrap().clone())
}

pub fn initialize(lua: LuaState) -> Result<()> {
  let atomic = AtomicFramework::new(lua)?;
  let engine_api_package = atomic.new_package(GROOM_PACKAGE)?;
  let logger = engine_api_package.logger();

  set_package_instance(engine_api_package.clone());
  set_logger(logger.clone());

  match setup_event_dispatcher(&engine_api_package) {
    Ok(_) => log("rust event dispatcher has been sucessfully set up"),
    Err(err) => log_with_level(LoggerLevel::Err, format!("failed set up event dispatcher: {err}")),
  };

  setup_api(lua, engine_api_package)
}

fn setup_api(lua: LuaState, engine_api_package: Package) -> Result<()> {
  engine_api_package.push_on_stack();

  lua_newtable(lua);
  lua_newtable(lua);

  lua_pushcfunction(lua, logic::api_execute_command);
  lua_setfield(lua, -2, cstr!("executeConsoleCommand"));

  lua_pushcfunction(lua, logic::api_detour_log);
  lua_setfield(lua, -2, cstr!("detourLogging"));

  lua_pushcfunction(lua, logic::api_hook_concommand);
  lua_setfield(lua, -2, cstr!("hookConCommand"));

  lua_pushcfunction(lua, logic::api_undetour_log);
  lua_setfield(lua, -2, cstr!("undetourLogging"));

  lua_pushcfunction(lua, logic::api_detour_concommands);
  lua_setfield(lua, -2, cstr!("detourConCommands"));

  lua_pushcfunction(lua, logic::api_undetour_concommands);
  lua_setfield(lua, -2, cstr!("undetourConCommands"));

  lua_setfield(lua, -2, cstr!("console"));

  lua_setfield(lua, -2, cstr!("api"));

  Ok(())
}