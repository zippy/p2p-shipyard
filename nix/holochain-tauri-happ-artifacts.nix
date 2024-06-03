{ craneLib, buildInputs, nativeBuildInputs, debug ? false }:
let
  src = craneLib.cleanCargoSource (craneLib.path ./reference-tauri-happ);
  commonArgs = {
    inherit src buildInputs nativeBuildInputs;

    doCheck = false;
    cargoExtraArgs = "";
    cargoCheckCommand = "";
    cargoBuildCommand =
      "cargo build --profile release --tests --locked --workspace";
  };
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    pname = "tauri-happ";
    version = "for-holochain-0.3.1-rc";
  });
in cargoArtifacts
