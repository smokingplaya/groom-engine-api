use anyhow::{Context, Result};
use atomicframework_api::{LuaReference, Package, string_to_ptr};
use std::sync::{Mutex, OnceLock, mpsc::{Receiver, Sender, channel}};
use rglua::{lua::{LuaState, REGISTRYINDEX, lua_call, lua_isfunction, lua_pop, lua_pushinteger, lua_pushnil, lua_pushstring, lua_rawgeti}, lua_function};
use crate::{common::get_package_instance, engine::classes::Color, utils::log_with_level};

pub type LoggingSystemLog = (Option<Color>, String);

#[derive(Debug)]
pub(super) struct LoggingSystemNode {
  pub timestamp: u64,
  pub content: LoggingSystemLog,
  pub prefix: Option<String>,
  pub thread_name: Option<String>
}

#[derive(Debug)]
pub(super) enum LuaChannelEvent {
  Log(LoggingSystemNode),
  Command(LuaReference, String, Vec<String>)
}

static LUA_THREAD_SENDER: OnceLock<Sender<LuaChannelEvent>> = OnceLock::new();
static LUA_THREAD_RECEIVER: OnceLock<Mutex<Receiver<LuaChannelEvent>>> = OnceLock::new();

pub(super) fn send_event(message: LuaChannelEvent) -> Result<()> {
	LUA_THREAD_SENDER
		.get()
		.context("sender not initialized")?
		.send(message)?;

	Ok(())
}

fn recv_msg() -> Option<LuaChannelEvent> {
	let rx = LUA_THREAD_RECEIVER.get()?;
	let guard = rx.lock().ok()?;

	guard.try_recv().ok()
}

/// At every tick of the game, mpsc checks the channel for new messages,
/// and if there are any, it creates an event in the `groom-engine-api` package.
#[lua_function]
fn on_think(lua: LuaState) -> i32 {
  let package = get_package_instance();

  while let Some(message) = recv_msg() {
    match message {
      LuaChannelEvent::Command(lua_func_ref, command, args) => {
        lua_rawgeti(lua, REGISTRYINDEX, lua_func_ref);

        if !lua_isfunction(lua, -1) {
          lua_pop(lua, 1);
          log_with_level(atomicframework_api::LoggerLevel::Err, "invalid lua reference (LuaChannelEvent::Command)");

          return 0;
        }

        lua_pushstring(lua, string_to_ptr!(command));

        let count = args.len();

        args.into_iter()
          .for_each(|arg| lua_pushstring(lua, string_to_ptr!(arg)));

        lua_call(lua, 1 + count as i32, 0);
      }

      LuaChannelEvent::Log(log) => {
        package.emit_event("onEngineLog", |state| {
          let (color, content) = log.content;

          // timestamp, color?, content, prefix?, thread?
          lua_pushinteger(state, log.timestamp as isize); // fixme: 2038 year problem, but well current lua in gmod does not support 64 bits integers so

          if let Some(color) = color {
            color.push_on_stack_safe(state);
          } else {
            lua_pushnil(state);
          }

          lua_pushstring(state, string_to_ptr!(content));

          if let Some(prefix) = log.prefix {
            lua_pushstring(state, string_to_ptr!(prefix));
          } else {
            lua_pushnil(state);
          }

          if let Some(thread_name) = log.thread_name {
            lua_pushstring(state, string_to_ptr!(thread_name));
          } else {
            lua_pushnil(state);
          }

          5
        }).ok();
      }
    };
  }

  0
}

pub fn setup_event_dispatcher(package: &Package) -> Result<()> {
  let (tx, rx) = channel();

	LUA_THREAD_SENDER
		.set(tx)
		.expect("sender already initialized");

	LUA_THREAD_RECEIVER
		.set(Mutex::new(rx))
		.expect("receiver already initialized");

  package.listen(on_think, "Think")
}