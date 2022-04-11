* Building GM8Emulator / GM8Decompiler
Start by cloning the repository. We use some git submodules at the moment (unfortunately), so make sure to clone with submodules.

#+begin_src sh
  git clone --recurse-submodules https://github.com/OpenGMK/OpenGMK.git
#+end_src

You can also recursively initialise the submodules after you've already cloned.

#+begin_src sh
  git submodule update --init --recursive
#+end_src

This project is written in the [[https://www.rust-lang.org/][Rust]] programming language. You can download the toolchain manager directly from [[https://rustup.rs/]] or a package manager of your choice. Our current minimum supported rust version (MSRV) policy is version *1.58*, if you're downloading it at the time of writing then you almost definitely are up to date but you can check with =rustc -V= to be sure. Once that's set up, building everything in release mode is pretty simple (but might take a while the first time).

#+begin_src sh
  cd path/to/repo-folder
  cargo build --release
#+end_src

The build artifacts will be located in =<repo-folder>/target/release= including libraries and binaries.
** Native DLLs for 64-bit Windows
If you're on Windows 64-bit and would like to play games with GM8Emulator that require 32-bit DLLs to function such as /GMFMODSimple/ or /supersound/ you'll also need to build the WoW64 server, preferably in the release profile. It requires the additional installation of the =i686-pc-windows-msvc= toolchain with rustup and you will need to build it separately.

#+begin_src sh
  rustup target add i686-pc-windows-msvc
  cd path/to/repo-folder/gm8emulator-wow64
  cargo build --target=i686-pc-windows-msvc --release
#+end_src

The build artifacts for the WoW64 server will be located in =<repo-folder>/gm8emulator-wow64/target/i686-pc-windows-msvc/release=. The binary should either be manually copied to the same folder as =gm8emulator.exe= to work, or the =OPENGMK_WOW64_BINARY= environment variable should be set with the path to the binary.

A much easier alternative to this is building the project as 32-bit on Windows, where the WoW64 server is not required and the DLL loading logic is bundled inside GM8Emulator. It should be noted that cross-platform extension emulation is planned for the long-term future.