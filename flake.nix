{
  description = "Bevy Game development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    # nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nixgl = {
      url = "github:guibou/nixGL";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, nixgl }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgsOrig = import nixpkgs {inherit system; };
        forced = builtins.trace "Current system is: ${system}" null;
        isLinux = (builtins.match ".*linux.*" system) != null;
        isDarwin = (builtins.match ".*darwin.*" system) != null;
        isImpure = builtins.hasAttr "currentTime" builtins;

        pkgs = import nixpkgs {
          inherit system;
          overlays = if (isLinux && isImpure) 
            then [ nixgl.overlays.default ]
            else [ ];
        };
        
        linuxPackages = with pkgs; [
          alsa-lib
          alsa-plugins
          udev
        ];
        linuxImpurePackages = with pkgs; [
          pkgs.nixgl.auto.nixGLDefault
        ];

        sdkVersion = "15";
        appleSDK = pkgs."apple-sdk_${sdkVersion}";
        appleLibs = if isDarwin then ([
          appleSDK
        ]) else [];

        # Platform-specific packages
        platformPackages = 
          (if (isLinux && !isImpure) then
            linuxPackages
          else if isLinux then
            linuxPackages ++ linuxImpurePackages 
          else if isDarwin then
            appleLibs
          else
            [ ]);
        
        # Common bindgen configuration
        commonBindgenIncludes = with pkgs; [
          ''-I"${llvmPackages_latest.libclang.lib}/lib/clang/${llvmPackages_latest.libclang.version}/include"''
        ];

        # Platform-specific bindgen configuration
        platformBindgenInputs = if isLinux then
          with pkgs; [
            glibc.dev
          ]
        else [];
        
        platformBindgenIncludes = if isLinux then
          with pkgs; [
            ''-I"${glib.dev}/include/glib-2.0"''
            ''-I${glib.out}/lib/glib-2.0/include/''
          ]
        else [];

        # It's standard in nix to pin the versions for reproducibility; here's where we
        # pin the Rust toolchain.
        overrides = (builtins.fromTOML (builtins.readFile (self + "/nix/rust-toolchain.toml")));
        
        # Platform-specific shell hook
        platformShellHook = if isLinux then ''
          alias gl='nixGL'
          export ALSA_PLUGIN_DIR=${pkgs.alsa-plugins}/lib/alsa-lib
          ''
        else
          "";
          
        libPath = with pkgs; lib.makeLibraryPath [
          # load external libraries that you need in your rust project here
        ];
      in
      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = with pkgs; [
            clang
            llvmPackages.bintools
            # shouldn't need this, but rustup (andt thus cargo), is too old for 2024 edition requirements:
            # https://github.com/NixOS/nixpkgs/pull/397177
            cargo
            rustup

            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
          ] ++ platformPackages;

          RUSTC_VERSION = overrides.toolchain.channel;
          RUSTC_COMPONENTS = nixpkgs.lib.escapeShellArgs (nixpkgs.lib.concatMap (c: ["-c" c]) overrides.toolchain.components);
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
          FORCED = forced;
          shellHook = ''
            rustup toolchain install $RUSTC_VERSION $RUSTC_COMPONENTS
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/:$PATH
            ${platformShellHook}
          '';

          # Add precompiled library to rustc search path
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here (e.g. pkgs.libvmi)
          ]);
          
          LD_LIBRARY_PATH = 
            (pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs)) + 
            # packages with non-standard lib paths:
            (if isLinux then ":${pkgs.alsa-plugins}/lib/alsa-lib" else "");

          # Add clang and other headers to bindgen search path
          BINDGEN_EXTRA_CLANG_ARGS =
            (builtins.map (a: ''-I"${a}/include"'') platformBindgenInputs)
            ++ commonBindgenIncludes
            ++ platformBindgenIncludes;
        };
      }
    );
}
