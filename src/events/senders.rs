use std::time::{SystemTime, UNIX_EPOCH};
use crate::events::{LuaChannelEvent, channel::{LoggingSystemLog, LoggingSystemNode, send_event}};

/// Pushes command to mpsc channel
#[allow(unused)]
pub fn send_command(lua_func_ref: i32, command: String, arguments: Vec<String>) -> bool {
  send_event(LuaChannelEvent::Command(lua_func_ref, command, arguments))
    .is_ok()
}

/// Pushes log to mpsc channel
#[allow(unused)]
pub fn send_log(thread_name: Option<String>, prefix: Option<String>, content: LoggingSystemLog) -> bool {
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

  send_event(LuaChannelEvent::Log(LoggingSystemNode {
    timestamp,
    prefix,
    content,
    thread_name
  })).is_ok()
}