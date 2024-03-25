{
  description = "Holochain";
  
  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/weekly";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
    android-nixpkgs = {
      url = "github:tadfisher/android-nixpkgs/stable";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    hcInfra.url = "github:holochain-open-dev/infrastructure";
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain.devShells;
        perSystem =
          { inputs'
          , config
          , pkgs
          , system
          , lib
          , ...
          }: rec {
            devShells.tauriDev = pkgs.mkShell {
              packages = with pkgs; [
                nodejs_20
              ];

              buildInputs = (with pkgs; [
                openssl
                # this is required for glib-networking
                glib
              ])
              ++ (lib.optionals pkgs.stdenv.isLinux
                (with pkgs; [
                  webkitgtk_4_1.dev
                  gdk-pixbuf
                  gtk3
                  # Video/Audio data composition framework tools like "gst-inspect", "gst-launch" ...
                  gst_all_1.gstreamer
                  # Common plugins like "filesrc" to combine within e.g. gst-launch
                  gst_all_1.gst-plugins-base
                  # Specialized plugins separated by quality
                  gst_all_1.gst-plugins-good
                  gst_all_1.gst-plugins-bad
                  gst_all_1.gst-plugins-ugly
                  # Plugins to reuse ffmpeg to play almost every video format
                  gst_all_1.gst-libav
                  # Support the Video Audio (Hardware) Acceleration API
                  gst_all_1.gst-vaapi
                  libsoup_3
                ]))
              ++ lib.optionals pkgs.stdenv.isDarwin
                (with pkgs; [
                  darwin.apple_sdk.frameworks.Security
                  darwin.apple_sdk.frameworks.CoreServices
                  darwin.apple_sdk.frameworks.CoreFoundation
                  darwin.apple_sdk.frameworks.Foundation
                  darwin.apple_sdk.frameworks.AppKit
                  darwin.apple_sdk.frameworks.WebKit
                  darwin.apple_sdk.frameworks.Cocoa
                ])
              ;

              nativeBuildInputs = (with pkgs; [
                perl
                pkg-config
                makeWrapper
              ])
              ++ (lib.optionals pkgs.stdenv.isLinux
                (with pkgs; [
                  wrapGAppsHook
                ]))
              ++ (lib.optionals pkgs.stdenv.isDarwin [
                pkgs.xcbuild
                pkgs.libiconv
              ]);

              shellHook = ''
                export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
                export GIO_EXTRA_MODULES=${pkgs.glib-networking}/lib/gio/modules
                export WEBKIT_DISABLE_COMPOSITING_MODE=1
              '';

            };

            devShells.androidDev = 
              let 
                androidPkgs = import pkgs.path {
                  inherit system;
                  config = {
                    android_sdk.accept_license = true;
                    allowUnfree = true;
                  };
                };
                android-sdk = inputs.android-nixpkgs.sdk.${system} (sdkPkgs: with sdkPkgs; [
                  cmdline-tools-latest
                  build-tools-30-0-3
                  platform-tools
                  ndk-bundle
                  platforms-android-33
                  # emulator
                  # system-images-android-33-google-apis-playstore-x86-64
                ]);
              in
                pkgs.mkShell {
                  packages = [ 
                    android-sdk 
                    androidPkgs.android-studio
                    pkgs.gradle
                    pkgs.jdk17
                  ];

                  shellHook = ''
                    export NDK_HOME=$ANDROID_SDK_ROOT/ndk-bundle
                  '';
                };

            devShells.tauriAndroidDev = 
              let 
                overlays = [ (import inputs.rust-overlay) ];
                rustPkgs = import pkgs.path {
                  inherit system overlays;
                };
                rust = rustPkgs.rust-bin.stable.latest.default.override {
                  extensions = [ "rust-src" ];
                  targets = [ 
                    "armv7-linux-androideabi"
                    "x86_64-linux-android"
                    "i686-linux-android"
                    "aarch64-unknown-linux-musl"
                    "wasm32-unknown-unknown"
                    "x86_64-pc-windows-gnu"
                    "x86_64-unknown-linux-musl"
                    "x86_64-apple-darwin"
                    "aarch64-linux-android"
                  ];
                };
              in
                pkgs.mkShell {
                  inputsFrom = [ devShells.tauriDev devShells.androidDev ];
                  packages = [ 
                    rust 
                  ];

                  shellHook = ''
                    export RUSTFLAGS+=" -C link-arg=$(gcc -print-libgcc-file-name)"
                  '';
                };

            
            devShells.holochainTauriAndroidDev = pkgs.mkShell {
              inputsFrom = [
                inputs'.holochain.devShells.holonix
                devShells.tauriAndroidDev
              ];
            };
            
            devShells.holochainTauriDev = pkgs.mkShell {
              inputsFrom = [
                inputs'.holochain.devShells.holonix
                devShells.tauriDev
              ];
            };

            devShells.default = pkgs.mkShell {
              inputsFrom = [
                devShells.holochainTauriAndroidDev
              ];
              packages = [
                pkgs.nodejs_20
                inputs'.hcInfra.packages.pnpm
              ];
            };
          };
      };
}

