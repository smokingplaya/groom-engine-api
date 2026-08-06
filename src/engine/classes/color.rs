use anyhow::{Result, bail};
use atomicframework_api::string_to_ptr;
use rglua::lua::{LuaState, lua_call, lua_getglobal, lua_isfunction, lua_isnil, lua_pop, lua_pushinteger, lua_pushnil};
use std::fmt;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Color(u8, u8, u8, u8);

#[allow(unused)]
impl Color {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self::new_with_alpha(r, g, b, 255)
  }

  pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self(r, g, b, a)
  }
}

#[allow(unused)]
impl Color {
  pub fn r(&self) -> u8 {
    self.0
  }

  pub fn g(&self) -> u8 {
    self.1
  }

  pub fn b(&self) -> u8 {
    self.2
  }

  pub fn a(&self) -> u8 {
    self.3
  }

  pub fn push_on_stack(&self, lua: LuaState) -> Result<()> {
    lua_getglobal(lua, string_to_ptr!("Color"));

    if !lua_isfunction(lua, -1) {
      lua_pop(lua, 1);
      bail!("Color is not global function")
    }

    lua_pushinteger(lua, self.r() as isize);
    lua_pushinteger(lua, self.g() as isize);
    lua_pushinteger(lua, self.b() as isize);
    lua_pushinteger(lua, self.a() as isize);

    lua_call(lua, 4, 1);

    if lua_isnil(lua, -1) {
      lua_pop(lua, 1);
      bail!("failed to create new Color instance")
    }

    Ok(())
  }

  pub fn push_on_stack_safe(&self, lua: LuaState) {
    if let Err(err) = self.push_on_stack(lua) {
      lua_pushnil(lua);
    }
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Color #{:x}{:x}{:x}{:x}", self.0, self.1, self.2, self.3)
  }
}