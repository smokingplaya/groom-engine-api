fn main() {
	println!("cargo:rustc-link-search=native=/home/athens/.steam/steam/steamapps/common/GarrysMod/bin/linux64");
	// println!("cargo:rustc-link-lib=tier0");
	println!("cargo:rustc-link-lib=tier0_client");
}