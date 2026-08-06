use rglua::interface::EngineClient;
use crate::define_source_interface;

#[cfg(target_os = "windows")]
define_source_interface!(EngineClient, ENGINE_CLIENT);
#[cfg(not(target_os = "windows"))]
define_source_interface!("engine_client.so", "VEngineClient015", EngineClient, ENGINE_CLIENT);