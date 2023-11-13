# libpreflate-sys

Minimal low-level Rust bindings to the [preflate](https://github.com/deus-libri/preflate) C++ library using [CXX](https://cxx.rs/).

## Disclaimer

This repo contains a modified copy of the preflate library v0.3.5 in the `vendor/preflate` directory. The copy is taken 
from the [precomp-cpp](https://github.com/schnaader/precomp-cpp) project as they modified the library to build under
Linux. Original CMakeLists.txt from [preflate](https://github.com/deus-libri/preflate) was added and modified with the following modifications:

- Changed minimum required CMake from 3.2 to 3.5 - CMake refused to build the project with 3.2
- Commented out the executable targets
- Added an "install" target which places the static library in `lib/` after building (For easier linking to Rust code)

Both precomp-cpp and preflate are licensed under Apache-2.0