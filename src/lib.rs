use atomicframework_api::LoggerLevel;
use rglua::{gmod_close, gmod_open, lua::{LuaState, lua_pop}};
use crate::{common::initialize, logic::{api_undetour_concommands, api_undetour_log}, utils::{log, log_with_level}};

// atomic framework package registration & lua api setup
mod common;
// declaring & implementing source engine (gmod) classes/interfaces
mod engine;
// main logic of this crate: console detouring, and such things
mod logic;
// mpsc (needed because of Lua is singlethreaded, so we need to)
mod events;
// some util functions, e.g logging
mod utils;

#[gmod_open]
fn gmod13_open(lua: LuaState) -> i32 {
  // ᕦ( ͡°﻿ ͜ʖ ͡°)ᕤ
  lua_pop(lua, 1);

  match initialize(lua) {
    Ok(_) => log("initialized successfully"),
    Err(err) => log_with_level(LoggerLevel::Err, format!("failed to initialize: {err}"))
  };

  0
}

#[gmod_close]
fn gmod_close(lua: LuaState) -> i32 {
  api_undetour_log(lua);
  api_undetour_concommands(lua);

  0
}