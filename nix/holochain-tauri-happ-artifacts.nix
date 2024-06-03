{ craneLib, buildInputs, nativeBuildInputs, debug ? false }:
let
  src = craneLib.cleanCargoSource (craneLib.path ./reference-tauri-happ);
  commonArgs = {
    inherit src buildInputs nativeBuildInputs;
    CARGO_PROFILE = "release";

    strictDeps = true;
    doCheck = false;
    cargoExtraArgs = "--workspace --tests";
  };
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    pname = "tauri-happ";
    version = "for-holochain-0.3.1-rc";
  });
in cargoArtifacts
