{ ... }:

{
  perSystem = { inputs', lib, pkgs, self', ... }: rec {
    packages.tauri-cli = pkgs.rustPlatform.buildRustPackage rec {
      pname = "tauri-cli";
      version = "2.0.0-beta.20";

      src = pkgs.fetchCrate {
        inherit pname version;
        hash = "sha256-5uhJIxqq3wG6FCZIAh7nITecwmlUZ82XlDFyLITSwxc=";
      };

      cargoHash = "sha256-62QLBdB8AWPTKqWBs8ejx407AO17DrLdPdM/jIlEzbI=";
      cargoDepsName = pname;
    };
  };
}