// ( ͡° ͜ʖ ͡°)
use anyhow::{Context, Result};
use atomicframework_api::{ptr_to_string, string_to_ptr, write_to_dumpfile};
use retour::{StaticDetour, static_detour};
use rglua::lua::LuaState;
use std::{collections::HashMap, sync::{LazyLock, Mutex}};
use crate::{call_interface_function, engine::interfaces::vstdlib::{CCommand, ConCommand, INTERFACE_VSTDLIB_CONVAR}, events::send_command};

// table (hashmap) that will be contain lua functions references
//
// HashMap:
//  clear = 32i32,
//  quit = 9482i32
//  >⩊< = 0i32
static COMMANDS_LOOKUP_TABLE: Mutex<LazyLock<HashMap<String, i32>>> = Mutex::new(LazyLock::new(HashMap::new));

pub fn hook_command(_lua: LuaState, command_name: *const i8, callback_luaref: i32) {
  if let Ok(mut guard) = COMMANDS_LOOKUP_TABLE.lock() {
    let _prev_callback_ref = guard.insert(ptr_to_string!(command_name), callback_luaref);

    // fixme this creates troubles if refs are the same

    // if let Some(prev_ref) = prev_callback_ref {
    //   if callback_luaref != prev_ref {
    //     luaL_unref(lua, REGISTRYINDEX, prev_ref);
    //   }
    // }
  }
}

// (づ｡◕‿‿◕｡)づ
// `dispatch` (virtual method of source engine's class ConCommand) method detour zone

type DispatchVirtualMethodSignature = extern "C" fn (*mut ConCommand, *mut CCommand);

// contains pointer to original `dispatch` function
// todo: remove mutex nahui otsuda
static DETOUR_FUNC: Mutex<LazyLock<Option<&StaticDetour<DispatchVirtualMethodSignature>>>> = Mutex::new(LazyLock::new(|| None));

// set value to `DETOUR_FUNC`
fn set_detour_func(func: &'static StaticDetour<DispatchVirtualMethodSignature>) {
  let mut detoured = false;
  if let Ok(opt) = DETOUR_FUNC.lock() {
    detoured = opt.is_some();
  }

  if !detoured
    && let Ok(mut guard) = DETOUR_FUNC.lock() {
      **guard = Some(func);
  }
}

// default con command handler
fn default_handler(this: *mut ConCommand, command: *mut CCommand) {
  if let Ok(detour_opt) = DETOUR_FUNC.lock()
    && let Some(detour) = **detour_opt {
    unsafe {
      #[allow(unused)]
      detour.disable();
      ConCommand::dispatch(&mut *this, command);
      #[allow(unused)]
      detour.enable();
    }
  }
}

// custom con command handler
//
// flow (x: String):
// 1. looks into COMMAND_LOOKUP_TABLE
//  1.1 if COMMAND_LOOKUP_TABLE[x] ~= nil -> send message about concommand has been used to the lua state
//  1.2 else uses default source engine's command handler
fn dispatch_handler(this: *mut ConCommand, command: *mut CCommand) {
  let name = unsafe {&mut *command}.get_name().unwrap_or_default().to_str().unwrap_or("siske world :S").to_string();

  if let Ok(guard) = COMMANDS_LOOKUP_TABLE.lock() {
    if let Some(lua_ref) = guard.get(&name) {
      unsafe {
        // send message to lua over mpsc
        send_command(*lua_ref, name, (*command).get_arguments());
      }

      return
    }
  }

  default_handler(this, command);
}

static_detour! {
  // #[cfg(target_os="windows")]
  static ConCommandDispatchMethod: extern "C" fn(*mut ConCommand, *mut CCommand);
  // #[cfg(not(target_os="windows"))]
  // static ConCommandDispatchMethod: extern "thiscall" fn(*mut ConCommand, *mut CCommand);
}

// 1. get any console command instance (pointer to it) (because vtable is same for all instances)
// 2. get pointer to dispatch function (ConCommand->vptr->dispatch)
// 3. change pointer of function `dispatch` (#14) in virtual table (ConCommand->vptr) to custom (fn dispatch_handler)
// ( ╹ -╹)?
pub fn detour() -> Result<()> {
  let cmd = get_cmd()
    .context("unable to find console command `clear`")?;

  let detour;

  // absolutely safe (100%)
  // blya budu (net)
  unsafe {
    let vptr = (*cmd).vtable as *const extern "C" fn (_this: *mut ConCommand, _cmd: *mut CCommand);
    // reading `ConCommand->vptr->dispatch` function
    #[cfg(target_os="windows")]
    let dispatch_func = vptr.add(14).read();
    #[cfg(target_os="linux")]
    let dispatch_func = vptr.add(15).read();

    // (☞ ͡° ͜ʖ ͡°)☞
    detour = ConCommandDispatchMethod.initialize(dispatch_func, dispatch_handler)?;
    detour.enable()?;
  }

  // put original func pointer into static variable
  set_detour_func(detour);

  Ok(())
}

// calls VEngineCvar007->FindCommand("clear")
//
// returns ptr of concommand `clear` instance
// why `clear`, not some other command?
// because `clear` is default command that 100% would
// exists on game initialization, so basically we can use any command
fn get_cmd() -> Option<*mut ConCommand> {
  call_interface_function!(INTERFACE_VSTDLIB_CONVAR, find_command, string_to_ptr!("clear"))
}

pub fn undetour() {
  // ti na skoupe kurochka
  // ( -_•)ᡕᠵデᡁ᠊╾━💥
  if let Ok(detour_opt) = DETOUR_FUNC.lock()
    && let Some(detour) = **detour_opt {
    unsafe {
      detour.disable()
        .ok();
    }
  }
}