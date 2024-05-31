{ craneLib, debug ? false }:
let
  src = craneLib.cleanCargoSource (craneLib.path ./../examples/end-user-happ);
  cargoVendorDir = craneLib.vendorCargoDeps { inherit src; };
  commonArgs = {
    inherit src cargoVendorDir;
    doCheck = false;
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
    CARGO_PROFILE = if debug then "debug" else "release";
    cargoExtraArgs = "--offline";
    cargoBuildCommand = ''
      RUSTFLAGS="--remap-path-prefix $(pwd)=/build/source/ --remap-path-prefix ${cargoVendorDir}=/build/source/" cargo build'';
  };
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    pname = "tauri-app";
    version = "for-holochain-0.3.1-rc";
  });

in { inherit cargoVendorDir cargoArtifacts; }

