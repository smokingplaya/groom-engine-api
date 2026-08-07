#[cfg(target_os = "linux")]
fn main() {
  println!("cargo:rustc-link-search=native=/home/user/steam/steamapps/common/GarrysMod/bin/linux64");
  println!("cargo:rustc-link-lib=tier0_client");
}

#[cfg(target_os = "windows")]
fn main() {
  #[cfg(target_os = "windows")]
  println!("cargo:rustc-link-search=native=C:/Program Files (x86)/Steam/steamapps/common/GarrysMod/bin/win64");
	#[cfg(target_os = "windows")]
	println!("cargo:rustc-link-lib=tier0");
}