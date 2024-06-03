{ craneLib, buildInputs, nativeBuildInputs, debug ? false }:
let
  src = craneLib.cleanCargoSource (craneLib.path ./../examples/end-user-happ);
  commonArgs = {
    inherit src buildInputs nativeBuildInputs;

    doCheck = false;
    cargoExtraArgs = "";
    cargoPhaseCommand = "";
    cargoBuildCommand =
      "cargo build --profile release --tests --offline --workspace";
  };
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    pname = "tauri-happ";
    version = "for-holochain-0.3.1-rc";
  });
in cargoArtifacts

