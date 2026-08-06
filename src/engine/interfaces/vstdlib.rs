use std::ffi::{CStr, c_char};
use viable::vtable;
use crate::{define_source_interface, engine::classes::{CUtlString, CUtlVector}};

#[vtable]
pub struct VstdlibConVar {
  // in VEngineCvar004 it would be unregister_concommand=7 and find_command_base_internal=10, but gmod client uses fucking VEngineCvar007
  //
  // VEngineCvar007 Reference
  // https://github.com/advancedfx/advancedfx-prop/blob/7e18f9e3669a63453d0867d2d33bdba2206662c7/AfxHookSource/csgo/sdk_src/public/icvar.h#L66
  // (IAppSystem) https://github.com/advancedfx/advancedfx-prop/blob/7e18f9e3669a63453d0867d2d33bdba2206662c7/AfxHookSource/csgo/sdk_src/public/appframework/IAppSystem.h#L59
  #[offset(18)]
  find_command_internal: extern "C" fn(name: *const i8) -> *mut ConCommand,

  // todo
  // #[cfg(target_arch = "x86")]
  // #[offset(23)]
  // pub install_console_display_func: extern "C" fn(ptr: *mut IConsoleDisplayFunc)
}

impl VstdlibConVar {
  pub fn find_command(&mut self, command_name: *const i8) -> Option<*mut ConCommand> {
    let ptr = self.find_command_internal(command_name);

    if ptr.is_null() {
      return None
    }

    Some(ptr)
  }
}

// fix me : libvstdlib is linux only
define_source_interface!("libvstdlib_client.so", "VEngineCvar007", VstdlibConVar, VSTDLIB_CONVAR); // todo i'am here rn

// Reference
// https://github.com/nillerusr/source-engine/blob/ed8209cc35c61fbd8ddff8480962a01c981eef2f/public/tier1/convar.h#L204
#[repr(C)]
pub struct CCommand {
	argc: i32,
	argv0_size: i32,
	args_buffer: [i8; 512],
	argv_buffer: [i8; 512],
	argv_ptr: [*const i8; 64],
	source: i32,
}

impl CCommand {
	#[inline]
	pub fn argc(&self) -> usize {
		self.argc.max(0) as usize
	}

	#[inline]
	pub fn get_name_ptr(&self) -> *const c_char {
		self.argv_ptr
			.get(0)
			.copied()
			.unwrap_or(std::ptr::null())
	}

	#[inline]
	pub fn get_name(&self) -> Option<&CStr> {
		unsafe {
			let ptr = self.get_name_ptr();

			if ptr.is_null() {
				None
			} else {
				Some(CStr::from_ptr(ptr))
			}
		}
	}

	#[allow(unused)]
	#[inline]
	pub fn get_argument_ptr(&self, index: usize) -> Option<*const c_char> {
		if index >= self.argc() {
			return None;
		}

		Some(self.argv_ptr[index])
	}

	#[allow(unused)]
	#[inline]
	pub fn get_argument(&self, index: usize) -> Option<&CStr> {
		unsafe {
			let ptr = self.get_argument_ptr(index)?;

			if ptr.is_null() {
				None
			} else {
				Some(CStr::from_ptr(ptr))
			}
		}
	}

	#[allow(unused)]
	pub fn get_arguments_raw(&self) -> Vec<*const c_char> {
		let argc = self.argc();

		if argc <= 1 {
			return Vec::new();
		}

		self.argv_ptr[1..argc].to_vec()
	}

	pub fn get_arguments(&self) -> Vec<String> {
		let argc = self.argc();

		if argc <= 1 {
			return Vec::new();
		}

		self.argv_ptr[1..argc]
			.iter()
			.filter_map(|&ptr| {
				if ptr.is_null() {
					None
				} else {
					Some(unsafe {
						CStr::from_ptr(ptr)
							.to_string_lossy()
							.into_owned()
					})
				}
			})
			.collect()
	}

	#[allow(unused)]
	#[inline]
	pub fn get_command_string(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.args_buffer.as_ptr()) }
	}

	#[allow(unused)]
	#[inline]
	pub fn get_args_string(&self) -> &CStr {
		if self.argv0_size <= 0 {
			c""
		} else {
			unsafe {
				CStr::from_ptr(
					self.args_buffer
						.as_ptr()
						.add(self.argv0_size as usize),
				)
			}
		}
	}
}

#[vtable]
pub struct ConCommand {
  #[offset(7)]
  pub get_name: extern "C" fn() -> *const i8,
  #[offset(8)]
  pub get_help_text: extern "C" fn() -> *const i8,
  #[offset(13)]
  pub auto_complete_suggest: extern "C" fn(cmd: *mut CCommand, partial: *const i8, commands: &CUtlVector<CUtlString>),
  #[offset(14)]
  pub can_auto_complete: extern "C" fn(cmd: *mut CCommand) -> bool,
  #[offset(15)]
  pub dispatch: extern "C" fn(cmd: *mut CCommand)
}