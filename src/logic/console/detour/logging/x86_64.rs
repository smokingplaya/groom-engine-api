use anyhow::Result;
use std::sync::OnceLock;
use atomicframework_api::ptr_to_string;
use crate::{engine::linkinglibs::tier0::{LoggingContext, LoggingSystem_PopLoggingState, LoggingSystem_PushLoggingState, LoggingSystem_RegisterLoggingListener, LoggingSystem_UnregisterLoggingListener}, events::send_log};

static LOGGER: OnceLock<ILoggingListener> = OnceLock::new();

#[repr(C)]
pub struct ILoggingListenerVTable {
 	offset_to_top: isize,
	typeinfo: *const (),
	log: extern "C" fn(
		this: *mut ILoggingListener,
		context: *const LoggingContext,
		message: *const i8
	)
}

unsafe impl Sync for ILoggingListenerVTable {}
unsafe impl Send for ILoggingListenerVTable {}

#[repr(C)]
pub struct ILoggingListener {
	vtable: *const ()
}

unsafe impl Sync for ILoggingListener {}
unsafe impl Send for ILoggingListener {}

extern "C" fn log(
	_self: *mut ILoggingListener,
	context: *const LoggingContext,
	message: *const i8
) {
  let thread_name = std::thread::current()
    .name()
    .map(String::from);

  unsafe {
    send_log(thread_name, None, (Some(((*context).color).clone()), ptr_to_string!(message)));
  }
}

static VTABLE: ILoggingListenerVTable = ILoggingListenerVTable {
 	offset_to_top: 0,
	typeinfo: std::ptr::null(),
	log
};

impl ILoggingListener {
	pub fn new() -> Self {
		let vptr = unsafe {
			(std::ptr::addr_of!(VTABLE) as *const u8)
				.add(std::mem::offset_of!(ILoggingListenerVTable, log))
				as *const ILoggingListenerVTable
		};

		Self {
			vtable: vptr as *const (),
		}
	}
}

pub fn detour() -> Result<()> {
	let logger = LOGGER
		.get_or_init(ILoggingListener::new);

	unsafe {
		LoggingSystem_PushLoggingState(false, false);
		LoggingSystem_RegisterLoggingListener(logger as *const _ as *mut _);
	}

	Ok(())
}

pub fn undetour() -> Result<()> {
  let logger = LOGGER
		.get_or_init(ILoggingListener::new); // todo <-- we do not need to undetour if LOGGER is not initialized

	unsafe {
		LoggingSystem_UnregisterLoggingListener(logger as *const _ as *mut _);
		LoggingSystem_PopLoggingState(false);
	}

	Ok(())
}