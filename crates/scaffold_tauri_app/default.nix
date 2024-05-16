{ inputs, self, ... }:

{
  perSystem = { inputs', pkgs, system, lib, ... }: rec {
    packages.scaffold-tauri-app = let
      craneLib = inputs.crane.lib.${system};

      cratePath = ./.;

      cargoToml =
        builtins.fromTOML (builtins.readFile "${cratePath}/Cargo.toml");
      crate = cargoToml.package.name;

      commonArgs = {
        src = craneLib.path ../../.;
        doCheck = false;
        buildInputs = inputs.hc-infra.outputs.lib.holochainAppDeps.buildInputs {
          inherit pkgs lib;
        } ++ self.lib.tauriAppDeps.buildInputs { inherit pkgs lib; };
        nativeBuildInputs =
          (self.lib.tauriAppDeps.nativeBuildInputs { inherit pkgs lib; })
          ++ (inputs.hc-infra.outputs.lib.holochainAppDeps.nativeBuildInputs {
            inherit pkgs lib;
          });
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
        version = "workspace";
        pname = "workspace";
      });
    in craneLib.buildPackage (commonArgs // {
      pname = crate;
      version = cargoToml.package.version;
      inherit cargoArtifacts;
    });

    checks.scaffold-tauri-app = pkgs.runCommandLocal "test-scaffold-tauri-app" {
      buildInputs =
        [ inputs'.holochain.outputs.hc-scaffold packages.scaffold-tauri-app ];
    } ''
      hc scaffold --template lit web-app forum
      cd forum
      nix flake update
      nix develop

      npm i

      scaffold-tauri-app forum

      nix flake update
      nix develop

      npm i
      npm run build:happ
      npm run tauri build      
    '';
  };
}
