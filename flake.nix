{
  description = "Holochain";

  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/0_3_rc";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
    android-nixpkgs = {
      url = "github:tadfisher/android-nixpkgs/stable";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    hc-infra.url = "github:holochain-open-dev/infrastructure";
    scaffolding = {
      url = "github:holochain/scaffolding";
      # inputs.holochain.follows = "holochain";
    };
  };

  outputs = inputs@{ ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holochain.devShells;
      perSystem = { inputs', config, pkgs, system, lib, ... }: rec {
        devShells.tauriDev = pkgs.mkShell {
          packages = with pkgs; [
            nodejs_20
            (let
              overlays = [ (import inputs.rust-overlay) ];
              rustPkgs = import pkgs.path { inherit system overlays; };
              rust = rustPkgs.rust-bin.stable."1.75.0".default.override {
                extensions = [ "rust-src" ];
              };
            in rust)
            shared-mime-info
            gsettings-desktop-schemas
          ];

          buildInputs = (with pkgs; [
            openssl
            # this is required for glib-networking
            glib
          ]) ++ (lib.optionals pkgs.stdenv.isLinux (with pkgs; [
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
          ])) ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreServices
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.Foundation
            darwin.apple_sdk.frameworks.AppKit
            darwin.apple_sdk.frameworks.WebKit
            darwin.apple_sdk.frameworks.Cocoa
          ]);

          nativeBuildInputs = (with pkgs; [ perl pkg-config makeWrapper ])
            ++ (lib.optionals pkgs.stdenv.isLinux
              (with pkgs; [ wrapGAppsHook ]))
            ++ (lib.optionals pkgs.stdenv.isDarwin [
              pkgs.xcbuild
              pkgs.libiconv
            ]);

          shellHook = ''
            export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
            export GIO_EXTRA_MODULES=${pkgs.glib-networking}/lib/gio/modules
            export WEBKIT_DISABLE_COMPOSITING_MODE=1
            export RUSTFLAGS+=" -C link-arg=$(gcc -print-libgcc-file-name)"
            export XDG_DATA_DIRS=${pkgs.shared-mime-info}/share:${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          '';
        };

        devShells.androidDev = let
          androidPkgs = import pkgs.path {
            inherit system;
            config = {
              android_sdk.accept_license = true;
              allowUnfree = true;
            };
          };
          android-sdk = inputs.android-nixpkgs.sdk.${system} (sdkPkgs:
            with sdkPkgs; [
              cmdline-tools-latest
              build-tools-30-0-3
              platform-tools
              ndk-bundle
              platforms-android-33
              emulator
              system-images-android-33-google-apis-playstore-x86-64
            ]);
        in pkgs.mkShell {
          packages = [
            android-sdk
            androidPkgs.android-studio
            pkgs.gradle
            pkgs.jdk17
            pkgs.aapt
          ];

          shellHook = ''
            export GRADLE_OPTS="-Dorg.gradle.project.android.aapt2FromMavenOverride=${pkgs.aapt}/bin/aapt2";

            export NDK_HOME=$ANDROID_SDK_ROOT/ndk-bundle
            echo "no" | avdmanager -s create avd -n Pixel -k "system-images;android-33;google_apis_playstore;x86_64" --force
          '';
        };

        devShells.tauriAndroidDev = let
          overlays = [ (import inputs.rust-overlay) ];
          rustPkgs = import pkgs.path { inherit system overlays; };
          rust = rustPkgs.rust-bin.stable."1.75.0".default.override {
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
        in pkgs.mkShell {
          inputsFrom = [ devShells.androidDev devShells.tauriDev ];
          packages = [ rust ];

          shellHook = ''
            export RUSTFLAGS+=" -C link-arg=$(gcc -print-libgcc-file-name)"
          '';
        };

        devShells.holochainTauriAndroidDev = let
          overlays = [ (import inputs.rust-overlay) ];
          rustPkgs = import pkgs.path { inherit system overlays; };
          rust = rustPkgs.rust-bin.stable."1.75.0".default.override {
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
        in pkgs.mkShell {
          inputsFrom = [
            devShells.tauriDev
            devShells.androidDev
            inputs'.holochain.devShells.holonix
          ];
          packages = [ rust ];

          shellHook = ''
            export RUSTFLAGS+=" -C link-arg=$(gcc -print-libgcc-file-name)"
          '';
        };

        devShells.holochainTauriDev = let
          overlays = [ (import inputs.rust-overlay) ];
          rustPkgs = import pkgs.path { inherit system overlays; };
          rust = rustPkgs.rust-bin.stable."1.75.0".default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          };
        in pkgs.mkShell {
          inputsFrom =
            [ devShells.tauriDev inputs'.holochain.devShells.holonix ];
          packages = [ rust ];

          shellHook = ''
            export RUSTFLAGS+=" -C link-arg=$(gcc -print-libgcc-file-name)"
          '';
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            inputs'.hc-infra.devShells.synchronized-pnpm
            devShells.holochainTauriAndroidDev
          ];
        };

        packages.hc-scaffold = inputs.scaffolding.lib.wrapCustomTemplate {
          inherit pkgs system;
          customTemplatePath = ./templates/happ-open-dev;
        };

        packages.scaffold-tauri-app = let
          craneLib = inputs.crane.lib.${system};

          cratePath = ./crates/scaffold_tauri_app;

          cargoToml =
            builtins.fromTOML (builtins.readFile "${cratePath}/Cargo.toml");
          crate = cargoToml.package.name;

          buildInputs = (with pkgs; [
            openssl
            inputs'.holochain.packages.opensslStatic
            sqlcipher
          ]) ++ (lib.optionals pkgs.stdenv.isDarwin
            (with pkgs.darwin.apple_sdk_11_0.frameworks; [
              AppKit
              CoreFoundation
              CoreServices
              Security
              IOKit
            ]));
          commonArgs = {
            inherit buildInputs;
            doCheck = false;
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            nativeBuildInputs = (with pkgs; [
              makeWrapper
              perl
              pkg-config
              inputs'.holochain.packages.goWrapper
            ]) ++ lib.optionals pkgs.stdenv.isDarwin
              (with pkgs; [ xcbuild libiconv ]);
          };
        in craneLib.buildPackage (commonArgs // {
          pname = crate;
          version = cargoToml.package.version;
        });
      };
    };
}

