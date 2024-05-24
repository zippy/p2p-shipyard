{
  description = "Build cross-platform holochain apps and runtimes";

  inputs = {
    crane.url = "github:ipetkov/crane";

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
  };

  nixConfig = {
    extra-substituters = [
      "https://holochain-ci.cachix.org"
      "https://holochain-open-dev.cachix.org"
      "https://darksoil-studio.cachix.org"
    ];
    extra-trusted-public-keys = [
      "holochain-ci.cachix.org-1:5IUSkZc0aoRS53rfkvH9Kid40NpyjwCMCzwRTXy+QN8="
      "holochain-open-dev.cachix.org-1:3Tr+9in6uo44Ga7qiuRIfOTFXog+2+YbyhwI/Z6Cp4U="
      "darksoil-studio.cachix.org-1:UEi+aujy44s41XL/pscLw37KEVpTEIn8N/kn7jO8rkc="
    ];
  };

  outputs = inputs@{ ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake { inherit inputs; } rec {
      flake = {
        lib = {
          tauriAppDeps = {
            buildInputs = { pkgs, lib }:
              (with pkgs; [
                openssl
                # this is required for glib-networking
                glib
              ]) ++ (lib.optionals pkgs.stdenv.isLinux (with pkgs; [
                webkitgtk_4_1
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
            nativeBuildInputs = { pkgs, lib }:
              (with pkgs; [ perl pkg-config makeWrapper ])
              ++ (lib.optionals pkgs.stdenv.isLinux
                (with pkgs; [ wrapGAppsHook ]))
              ++ (lib.optionals pkgs.stdenv.isDarwin [
                pkgs.xcbuild
                pkgs.libiconv
              ]);
          };
        };
      };

      imports = [
        ./crates/scaffold-tauri-app/default.nix
        ./crates/scaffold-holochain-runtime/default.nix
        ./custom-go-compiler.nix
      ];

      systems = builtins.attrNames inputs.holochain.devShells;
      perSystem = { inputs', config, self', pkgs, system, lib, ... }: rec {
        devShells.tauriDev = pkgs.mkShell {
          packages = with pkgs; [
            nodejs_20
            packages.tauriRust
            shared-mime-info
            gsettings-desktop-schemas
          ];

          buildInputs =
            flake.lib.tauriAppDeps.buildInputs { inherit pkgs lib; };

          nativeBuildInputs =
            flake.lib.tauriAppDeps.nativeBuildInputs { inherit pkgs lib; };

          shellHook = ''
            export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
            export GIO_EXTRA_MODULES=${pkgs.glib-networking}/lib/gio/modules
            export WEBKIT_DISABLE_COMPOSITING_MODE=1
            export XDG_DATA_DIRS=${pkgs.shared-mime-info}/share:${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          '';
        };

        devShells.androidDev = pkgs.mkShell {
          packages = [ packages.android-sdk pkgs.gradle pkgs.jdk17 pkgs.aapt ];

          shellHook = ''
            export GRADLE_OPTS="-Dorg.gradle.project.android.aapt2FromMavenOverride=${pkgs.aapt}/bin/aapt2";

            export NDK_HOME=$ANDROID_SDK_ROOT/ndk-bundle
          '';
        };

        devShells.androidEmulatorDev = let
          android-sdk = inputs.android-nixpkgs.sdk.${system} (sdkPkgs:
            with sdkPkgs; [
              emulator
              system-images-android-33-google-apis-playstore-x86-64
            ]);
        in pkgs.mkShell {
          inputsFrom = [ devShells.androidDev ];
          packages = [ android-sdk ];

          shellHook = ''
            echo "no" | avdmanager -s create avd -n Pixel -k "system-images;android-33;google_apis_playstore;x86_64" --force
          '';
        };

        devShells.tauriAndroidDev = let
          overlays = [ (import inputs.rust-overlay) ];
          rustPkgs = import pkgs.path { inherit system overlays; };
          rust = rustPkgs.rust-bin.stable."1.77.2".default.override {
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
        };

        packages.android-sdk = inputs.android-nixpkgs.sdk.${system} (sdkPkgs:
          with sdkPkgs; [
            cmdline-tools-latest
            build-tools-30-0-3
            platform-tools
            ndk-bundle
            platforms-android-33
          ]);

        packages.tauriRust = let
          overlays = [ (import inputs.rust-overlay) ];
          rustPkgs = import pkgs.path { inherit system overlays; };
          rust = rustPkgs.rust-bin.stable."1.77.2".default.override {
            extensions = [ "rust-src" ];
          };
          linuxCargo = pkgs.writeShellApplication {
            name = "cargo";
            runtimeInputs = [ rust ];
            text = ''
              RUSTFLAGS="-C link-arg=$(gcc -print-libgcc-file-name)" cargo "$@"
            '';
          };
          linuxRust = pkgs.symlinkJoin {
            name = "rust";
            paths = [ linuxCargo rust ];
          };
        in if pkgs.stdenv.isLinux then linuxRust else rust;

        packages.holochainTauriRust = let
          overlays = [ (import inputs.rust-overlay) ];
          rustPkgs = import pkgs.path { inherit system overlays; };
          rust = rustPkgs.rust-bin.stable."1.77.2".default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          };
          linuxCargo = pkgs.writeShellApplication {
            name = "cargo";
            runtimeInputs = [ rust ];
            text = ''
              RUSTFLAGS="-C link-arg=$(gcc -print-libgcc-file-name)" cargo "$@"
            '';
          };
          linuxRust = pkgs.symlinkJoin {
            name = "rust";
            paths = [ linuxCargo rust ];
          };
        in if pkgs.stdenv.isLinux then linuxRust else rust;

        packages.androidTauriRust = let
          overlays = [ (import inputs.rust-overlay) ];
          rustPkgs = import pkgs.path { inherit system overlays; };
          rust = rustPkgs.rust-bin.stable."1.77.2".default.override {
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
          linuxCargo = pkgs.writeShellApplication {
            name = "cargo";
            runtimeInputs = [ rust ];
            text = ''
              RUSTFLAGS="-C link-arg=$(gcc -print-libgcc-file-name)" cargo "$@"
            '';
          };
          linuxRust = pkgs.symlinkJoin {
            name = "rust";
            paths = [ linuxCargo rust packages.android-sdk ];
          };
          customZigbuildCargo = pkgs.writeShellApplication {
            name = "cargo";

            runtimeInputs =
              [ rust (pkgs.callPackage ./custom-cargo-zigbuild.nix { }) ];

            text = ''
              if [ "$#" -ne 0 ] && [ "$1" = "build" ]
              then
                cargo-zigbuild "$@"
              else
                cargo "$@"
              fi
            '';
          };
          darwinAndroidRust = pkgs.symlinkJoin {
            name = "darwin-rust-for-android";
            paths = [ customZigbuildCargo rust packages.android-sdk ];
            buildInputs = [ pkgs.makeWrapper ];
            postBuild = ''
              wrapProgram $out/bin/cargo \
                --set RUSTFLAGS "-L linker=clang" \
                --set RANLIB ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ranlib \
                --set CC_aarch64_linux_android ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android24-clang \
                --set CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android24-clang \
                --set CC_i686_linux_android ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android24-clang \
                --set CARGO_TARGET_I686_LINUX_ANDROID_LINKER ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android24-clang \
                --set CC_x86_64_linux_android ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android24-clang \
                --set CARGO_TARGET_x86_64_LINUX_ANDROID_LINKER ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android24-clang \
                --set CC_armv7_linux_androideabi ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi24-clang \
                --set CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER ${packages.android-sdk}/share/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi24-clang
            '';
          };
        in if pkgs.stdenv.isDarwin then darwinAndroidRust else linuxRust;

        devShells.holochainTauriAndroidDev = pkgs.mkShell {
          inputsFrom = [
            devShells.tauriDev
            devShells.androidDev
            inputs'.holochain.devShells.holonix
          ];
          packages =
            [ packages.androidTauriRust self'.packages.custom-go-wrapper ];
        };

        devShells.holochainTauriDev = pkgs.mkShell {
          inputsFrom =
            [ devShells.tauriDev inputs'.holochain.devShells.holonix ];
          packages = [ packages.holochainTauriRust ];
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            inputs'.hc-infra.devShells.synchronized-pnpm
            devShells.holochainTauriAndroidDev
          ];
        };
      };
    };
}

