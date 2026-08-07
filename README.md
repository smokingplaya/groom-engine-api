# Groom Engine API

## Building
1. Clone this repository
2. Open `build.rs` and change `rustc-link-search` path to path of yours installed game + `/bin` folder (i.g /home/user/steam/steamapps/common/GarrysMod/bin/linux32) (where dynamic libraries)

## If you are on windows
1. Open `lib/public/tier0.lib` file in `danielga/sourcesdk_minimal` in your browser - [x86](https://github.com/danielga/sourcesdk-minimal/blob/master/lib/public/tier0.lib) | [x86_64](https://github.com/danielga/sourcesdk-minimal/blob/x86-64-branch/lib/public/x64/tier0.lib)
2. Download `tier0.lib` file (it should be already opened)
3. Copy `tier0.lib` to your game binaries directory (GarrysMod/bin/linux32 etc)