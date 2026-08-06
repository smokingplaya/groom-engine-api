use crate::engine::classes::Color;
#[cfg(target_arch = "x86_64")]
use std::sync::LazyLock;
#[cfg(target_arch = "x86_64")]
use crate::logic::ILoggingListener;

#[allow(unused)]
#[repr(C)]
pub enum LoggingSeverity {
	Message,
	Warning,
	Assert,
	Error,
	HighestSeverity
}

#[allow(unused)]
#[repr(C)]
#[derive(Debug)]
pub enum LoggingResponse
{
	Continue,
	Debugger,
	Abort,
}

#[allow(unused)]
#[repr(C)]
pub enum LoggingChannelFlags {
	ConsoleOnly = 1,
	DoNotEcho
}

#[repr(C)]
pub struct LoggingContext {
	pub channel_id: i32,
	pub flags: LoggingChannelFlags,
	pub severity: LoggingSeverity,
	pub color: Color
}

#[allow(unused)]
#[link(name = "tier0_client")]
unsafe extern "C" {
  #[cfg(target_arch = "x86_64")]
  // pub fn LoggingSystem_RegisterLoggingListener(); // https://github.com/danielga/sourcesdk-minimal/blob/99cafecf352ef5e6af6b1dade2566a8680039152/public/tier0/logging.h#L720
  pub fn LoggingSystem_RegisterLoggingListener(listener: *mut ILoggingListener); // https://github.com/danielga/sourcesdk-minimal/blob/99cafecf352ef5e6af6b1dade2566a8680039152/public/tier0/logging.h#L720
  #[cfg(target_arch = "x86_64")]
  pub fn LoggingSystem_UnregisterLoggingListener(listener: *mut ILoggingListener); // https://github.com/danielga/sourcesdk-minimal/blob/99cafecf352ef5e6af6b1dade2566a8680039152/public/tier0/logging.h#L720
  #[cfg(target_arch = "x86_64")]
  pub fn LoggingSystem_PushLoggingState(thread_local: bool, clear_state: bool); // https://github.com/danielga/sourcesdk-minimal/blob/99cafecf352ef5e6af6b1dade2566a8680039152/public/tier0/logging.h#L726
  #[cfg(target_arch = "x86_64")]
  pub fn LoggingSystem_PopLoggingState(thread_local: bool); // https://github.com/danielga/sourcesdk-minimal/blob/99cafecf352ef5e6af6b1dade2566a8680039152/public/tier0/logging.h#L726
  #[cfg(target_arch = "x86_64")]
  pub fn LoggingSystem_LogDirect(channel_id: i32, sev: LoggingSeverity, color: Color, msg: *const i8) -> LoggingResponse;
  #[cfg(target_arch = "x86_64")]
	pub fn LoggingSystem_Log(channel: i32, severity: LoggingSeverity, fmt: *const i8, ...) -> LoggingResponse;
	#[cfg(target_arch = "x86_64")]
	#[link_name = "_Z17LoggingSystem_Logi17LoggingSeverity_t5ColorPKcz"]
	pub fn LoggingSystem_ColoredLog(channel: i32, severity: LoggingSeverity, color: Color, fmt: *const i8, ...) -> LoggingResponse;
	#[cfg(target_arch = "x86_64")]
  pub fn LoggingSystem_FindChannel(channel_name: *const i8) -> i32;
  #[cfg(target_arch = "x86_64")]
  pub fn LoggingSystem_IsChannelEnabled(channel_id: i32, severity: LoggingSeverity) -> bool;
}

#[allow(unused)]
pub static LOGGINGSYSTEM_CHANNEL_GENERAL: LazyLock<i32> = LazyLock::new(|| unsafe { LoggingSystem_FindChannel(rglua::cstr!("General")) });