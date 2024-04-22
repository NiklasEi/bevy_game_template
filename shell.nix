# Cf: https://nixos.wiki/wiki/Rust

# Variable and Imports definition
let
  # Import overlay to install specific Rust versions
  rust_overlay = import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  # Import the 23.11 stable Nixpkgs library and apply the rust packages overlay
  pkgs = import
    (fetchTarball
      "https://github.com/nixos/nixpkgs/tarball/nixos-23.11"
    )
    { overlays = [ rust_overlay ]; };
  # After importing the nixpkgs and applying the rust overlay we can customize our rust installation
  rust = (
    # We use the toolchain and components defines in the `rust-toolchain.toml` file. This file needs to exist.
    pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
  );
in

# Set all attributes of `pkgs` into our global scope
with pkgs;

# Create our shell environment
mkShell rec {
  # List all dependencies
  nativeBuildInputs = with pkgs; [
    cargo-apk
    cargo-audit
    cargo-binstall
    cargo-cache
    cargo-make
    cargo-nextest
    cargo-tarpaulin
    cargo-udeps
    cargo-xbuild
    clang
    clippy
    lld
    nixpkgs-fmt
    pkg-config
    rust
    trunk
    typos
  ];
  # List all necessary libraries
  buildInputs = [
    alsa-lib
    libxkbcommon
    udev
    vulkan-loader
    wayland # To use the wayland feature
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr # To use the x11 feature
  ];
  LIBCLANG_PATH = "${libclang.lib}/lib";
  RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
  # This is usually not recommended by the Nix community but is necessary for dynamic linking and quick iteration in Bevy builds
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
