#
# Adapted from: https://nixos.wiki/wiki/Rust
#
# let
#   nixgl = import (builtins.fetchTarball {
#     name = "NixGL";
#     url = https://github.com/guibou/nixGL/tarball/489d6b095ab9d289fe11af0219a9ff00fe87c7c5;
#     # Hash obtained using `nix-prefetch-url --unpack <url>`
#     sha256 = "03kwsz8mf0p1v1clz42zx8cmy6hxka0cqfbfasimbj858lyd930k";
#   }) {}; # TODO: nixGL WIP for this shell, may need to switch to flake
# in
{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      llvmPackages_latest.llvm
      llvmPackages_latest.bintools
      zlib.out
      pkg-config
      fontconfig
      freetype
      rustup
      llvmPackages_latest.lld
      # for wasm
      openssl
      # Only for building bracket-lib, I think:
      alsa-lib
      systemd
      # To enable faster linking in Rust projects
      clang
      mold
      # only for bevy
      cmake # needed via expat-sys
    ];
     # Only for building bracket-lib, I think:
    nativeBuildInputs = [ pkgs.pkg-config ];
    # TODO: change this to check if there's a rust-toolchain file, otherwise use
    # the current toolchain
    RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
    HISTFILE = toString ./.history;
    shellHook = ''
      export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
      export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.xorg.libX11}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.xorg.libXcursor}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.xorg.libXrandr}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.xorg.libXi}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.libGL}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.fontconfig.lib}/lib      
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.freetype}/lib
      '';
    # Add libvmi precompiled library to rustc search path
    RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
      # pkgs.libvmi
    ]);
    # Add libvmi, glibc, clang, glib headers to bindgen search path
    BINDGEN_EXTRA_CLANG_ARGS = 
    # Includes with normal include path
    (builtins.map (a: ''-I"${a}/include"'') [
      # pkgs.libvmi
      pkgs.glibc.dev 
    ])
    # Includes with special directory paths
    ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    ];

  }
