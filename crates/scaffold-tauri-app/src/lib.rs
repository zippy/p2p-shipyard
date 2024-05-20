use anyhow::Result;
use file_tree_utils::{dir_to_file_tree, map_file, FileTree, FileTreeError};
use handlebars::RenderErrorReason;
use holochain_scaffolding_utils::GetOrChooseWebAppManifestError;
use include_dir::{include_dir, Dir};
use nix_scaffolding_utils::{add_flake_input_to_flake_file, NixScaffoldingUtilsError};
use npm_scaffolding_utils::{
    add_npm_dev_dependency_to_package, add_npm_script_to_package, choose_npm_package,
    guess_or_choose_package_manager, NpmScaffoldingUtilsError,
};
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use templates_scaffolding_utils::{
    helpers::merge::register_merge, register_case_helpers, render_template_file_tree_and_merge_with_existing, TemplatesScaffoldingUtilsError
};
use thiserror::Error;

static TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/executable-happ");

#[derive(Error, Debug)]
pub enum ScaffoldExecutableHappError {
    #[error(transparent)]
    NpmScaffoldingUtilsError(#[from] NpmScaffoldingUtilsError),

    #[error(transparent)]
    RenderError(#[from] RenderErrorReason),

    #[error(transparent)]
    TemplatesScaffoldingUtilsError(#[from] TemplatesScaffoldingUtilsError),

    #[error(transparent)]
    GetOrChooseWebAppManifestError(#[from] GetOrChooseWebAppManifestError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    RegexError(#[from] regex::Error),

    #[error(transparent)]
    NixScaffoldingUtilsError(#[from] NixScaffoldingUtilsError),

    #[error(transparent)]
    DialoguerError(#[from] dialoguer::Error),

    #[error(transparent)]
    FileTreeError(#[from] FileTreeError),

    #[error("JSON serialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Malformed package.json {0}: {1}")]
    MalformedJsonError(PathBuf, String),
}

#[derive(Serialize, Deserialize, Debug)]
struct ScaffoldExecutableHappData {
    app_name: String,
}

pub fn scaffold_tauri_app(
    file_tree: FileTree,
    ui_package: Option<String>,
) -> Result<FileTree, ScaffoldExecutableHappError> {
    // - Detect npm package manager
    let package_manager = guess_or_choose_package_manager(&file_tree)?;

    // - Guess the name of the app -> from the web-happ.yaml file
    let (_web_happ_manifest_path, web_happ_manifest) =
        holochain_scaffolding_utils::get_or_choose_web_app_manifest(&file_tree)?;
    let app_name = web_happ_manifest.app_name().to_string();

    // let final_web_happ_path_from_root = web_happ_manifest_path.join(format!("{app_name}.webhapp"));

    // - Create the src-tauri directory structure
    let template_file_tree = dir_to_file_tree(&TEMPLATE)?;
    let h = handlebars::Handlebars::new();
    let h = register_case_helpers(h);
    let h = register_merge(h);

    let mut file_tree = render_template_file_tree_and_merge_with_existing(
        file_tree,
        &h,
        &template_file_tree,
        &ScaffoldExecutableHappData {
            app_name: app_name.clone(),
        },
    )?;
    // TODO: what about this?? - In lib.rs, change the name of the app
    // map_file(
    //    &mut file_tree,
    //    PathBuf::from("src-tauri/").as_path(),
    //    |package_json_content| {},
    // )?;

    let ui_package = match ui_package {
        Some(ui_package) => ui_package,
        None => choose_npm_package(&file_tree, &String::from("Which NPM package contains your UI?\n\nThis is needed so that the NPM scripts can start the UI and tauri can connect to it."))?,
    };

    // - In package.json
    // - Add "start", "network", "local-services", "build:zomes"
    let root_package_json_path = PathBuf::from("package.json");
    map_file(
        &mut file_tree,
        root_package_json_path.clone().as_path(),
        |package_json_content| {
            let package_json_content = add_npm_dev_dependency_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("@tauri-apps/cli"),
                &String::from("^2.0.0-beta.17"),
            )?;
            let package_json_content = add_npm_dev_dependency_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("concurrently"),
                &String::from("^8.2.2"),
            )?;
            let package_json_content = add_npm_dev_dependency_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("concurrently-repeat"),
                &String::from("^0.0.1"),
            )?;
            let package_json_content = add_npm_dev_dependency_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("internal-ip-cli"),
                &String::from("^2.0.0"),
            )?;
            let package_json_content = add_npm_dev_dependency_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("new-port-cli"),
                &String::from("^1.0.0"),
            )?;
            let package_json_content = add_npm_script_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("start"),
                &format!(
                    "AGENTS=2 {}",
                    package_manager.run_script_command("network".into(), None)
                ),
            )?;
            let package_json_content = add_npm_script_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("network"),
                &format!("{} && BOOTSTRAP_PORT=$(port) SIGNAL_PORT=$(port) INTERNAL_IP=$(internal-ip --ipv4) concurrently -k \"{}\" \"UI_PORT=1420 {}\" \"{}\"", 
                    
                    package_manager.run_script_command(String::from("build:happ"), None),
                    package_manager.run_script_command(String::from("local-services"), None ),
                    package_manager.run_script_command(String::from("start"), Some(ui_package.clone())),
                    package_manager.run_script_command(String::from("launch"), None)
                ),
            )?;
            let package_json_content = add_npm_script_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("local-services"),
                &format!("hc run-local-services --bootstrap-interface $INTERNAL_IP --bootstrap-port $BOOTSTRAP_PORT --signal-interfaces $INTERNAL_IP --signal-port $SIGNAL_PORT"),
            )?;

            let package_json_content = add_npm_script_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("android:network"),
                &format!("{} && BOOTSTRAP_PORT=$(port) SIGNAL_PORT=$(port) INTERNAL_IP=$(internal-ip --ipv4) concurrently -k \"{}\" \"UI_PORT=1420 {}\" \"{}\" \"{}\"",
                    package_manager.run_script_command(String::from("build:happ"), None),
                    package_manager.run_script_command(String::from("local-services"), None),
                    package_manager.run_script_command(String::from("start"), Some(ui_package.clone())),
                    package_manager.run_script_command(String::from("tauri dev"), None),
                    package_manager.run_script_command(String::from("tauri android dev"), None),
                ),
            )?;

            let package_json_content = add_npm_script_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("build:zomes"),
                &format!("CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown --workspace --exclude {app_name}"),
            )?;
            let package_json_content = add_npm_script_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("launch"),
                &format!(
                    "concurrently-repeat \"{}\" $AGENTS",
                    package_manager.run_script_command("tauri dev".into(), None)
                ),
            )?;
            add_npm_script_to_package(
                &(root_package_json_path.clone(), package_json_content),
                &String::from("tauri"),
                &String::from("tauri"),
            )
        },
    )?;

    // - In ui/package.json
    map_file(
        &mut file_tree,
        PathBuf::from("ui/package.json").as_path(),
        |ui_package_json| {
            let ui_package_json = add_npm_script_to_package(
                &(root_package_json_path.clone(), ui_package_json),
                &String::from("start"),
                &String::from("vite --clearScreen false"),
            )?;
            add_npm_dev_dependency_to_package(
                &(root_package_json_path.clone(), ui_package_json),
                &String::from("internal-ip"),
                &String::from("^7.0.0"),
            )
    })?;

    // - In flake.nix
    map_file(
        &mut file_tree,
        PathBuf::from("flake.nix").as_path(),
        |flake_nix_content| {
            // - If it exists or specialArgs exist, add the name of the app to the excludedCrates list
            let non_wasm_crates_re = Regex::new(r#"nonWasmCrates = \["#)?;

            let captures_iter: Vec<Captures<'_>> = non_wasm_crates_re
                .captures_iter(&flake_nix_content)
                .collect();

            let flake_nix_content = match captures_iter.len() {
                0 => {
                    let mk_flake_re = Regex::new(r#"mkFlake(\\n)*\s*\{"#)?;

                    mk_flake_re
                        .replace(
                            &flake_nix_content,
                            format!(
                                r#"mkFlake
    {{
      specialArgs.nonWasmCrates = [ "{app_name}" ];"#
                            ),
                        )
                        .to_string()
                }
                _ => non_wasm_crates_re
                    .replace(
                        &flake_nix_content,
                        format!("nonWasmCrates = [ \"{app_name}\" "),
                    )
                    .to_string(),
            };

            // - Add the `tauri-plugin-holochain` as input to the flake
            let flake_nix_content = add_flake_input_to_flake_file(
                flake_nix_content,
                String::from("tauri-plugin-holochain"),
                String::from("github:darksoil-studio/tauri-plugin-holochain"),
            )?;

            let scope_opener = String::from("devShells.default = pkgs.mkShell {");

            let (mut open, mut close) =
                get_scope_open_and_close_char_indexes(&flake_nix_content, &scope_opener)?;
            // Move the open character to the beginning of the line for the scope opener
            open -= scope_opener.len();
            while flake_nix_content.chars().nth(open).unwrap() == ' '
                || flake_nix_content.chars().nth(open).unwrap() == '\t'
            {
                open -= 1;
            }
            close += 2;

            // - Add an androidDev devshell by copying the default devShell, and adding the holochainTauriAndroidDev
            let android_dev_shell = flake_nix_content[open..close]
                .to_string()
                .clone()
                .replace("default", "androidDev")
                .replace(
                    "inputsFrom = [",
                    r#"inputsFrom = [
              inputs'.tauri-plugin-holochain.devShells.holochainTauriAndroidDev"#,
                );

            // - Add the holochainTauriDev to the default devShell
            let default_dev_shell = flake_nix_content[open..close].to_string().replace(
                "inputsFrom = [",
                r#"inputsFrom = [
              inputs'.tauri-plugin-holochain.devShells.holochainTauriDev"#,
            );

            let flake_nix_content = format!(
                "{}{}{}{}",
                &flake_nix_content[..open],
                default_dev_shell,
                android_dev_shell,
                &flake_nix_content[close..]
            );

            let result: Result<String, ScaffoldExecutableHappError> = Ok(flake_nix_content);
            result
        },
    )?;

    Ok(file_tree)
}

pub fn get_scope_open_and_close_char_indexes(
    text: &String,
    scope_opener: &String,
) -> Result<(usize, usize), RenderErrorReason> {
    let mut index = text
        .find(scope_opener.as_str())
        .ok_or(RenderErrorReason::Other(
            "Given scope opener not found in the given parameter".into(),
        ))?;

    index = index + scope_opener.len() - 1;
    let scope_opener_index = index.clone();
    let mut scope_count = 1;

    while scope_count > 0 {
        index += 1;
        match text.chars().nth(index) {
            Some('{') => {
                scope_count += 1;
            }
            Some('}') => {
                scope_count -= 1;
            }
            None => {
                return Err(RenderErrorReason::Other("Malformed scopes".into()));
            }
            _ => {}
        }
    }

    // let mut whitespace = true;

    // while whitespace {
    //     match text.chars().nth(index - 1) {
    //         Some(' ') => {
    //             index -= 1;
    //         }
    //         _ => {
    //             whitespace = false;
    //         }
    //     }
    // }

    Ok((scope_opener_index, index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use build_fs_tree::{dir, file};
    use file_tree_utils::file_content;

    #[test]
    fn simple_case_test() {
        let repo: FileTree = dir! {
            "flake.nix" => file!(default_flake_nix()),
            "workdir" => dir! {
                "web-happ.yaml" => file!(empty_web_happ_yaml("mywebhapp"))
            },
            "package.json" => file!(empty_package_json("root")),
            "package-lock.json" => file!(empty_package_json("root")),
            "packages" => dir! {
                "package1" => dir! {
                    "package.json" => file!(empty_package_json("package1"))
                },
                "package2" => dir! {
                    "package.json" => file!(empty_package_json("package2"))
                }
            }
        };

        let repo = scaffold_tauri_app(repo, Some(String::from("package1"))).unwrap();

        assert_eq!(
            file_content(&repo, PathBuf::from("package.json").as_path()).unwrap(),
            r#"{
  "name": "root",
  "dependencies": {},
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0-beta.17",
    "concurrently": "^8.2.2",
    "concurrently-repeat": "^0.0.1",
    "internal-ip-cli": "^2.0.0",
    "new-port-cli": "^1.0.0"
  },
  "scripts": {
    "start": "AGENTS=2 npm run network",
    "network": "BOOTSTRAP_PORT=$(port) SIGNAL_PORT=$(port) INTERNAL_IP=$(internal-ip --ipv4) concurrently -k \"npm run local-services\" \"npm run -w package1 start\" \"npm run launch\"",
    "local-services": "hc run-local-services --bootstrap-port $BOOTSTRAP_PORT --signal-port $SIGNAL_PORT",
    "build:zomes": "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown --workspace --exclude mywebhapp",
    "launch": "concurrently-repeat \"npm run tauri dev\" $AGENTS",
    "tauri": "tauri"
  }
}"#
        );

        assert_eq!(
            file_content(&repo, PathBuf::from("flake.nix").as_path()).unwrap(),
            r#"{
  description = "Template for Holochain app development";
  
  inputs = {
    tauri-plugin-holochain.url = "github:darksoil-studio/tauri-plugin-holochain";
    nixpkgs.follows = "holochain/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/weekly";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
    hc-infra.url = "github:holochain-open-dev/utils";
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
    {
      specialArgs.nonWasmCrates = [ "mywebhapp" ];
      inherit inputs;
      specialArgs = {
        rootPath = ./.;
      };
    }
    {

      systems = builtins.attrNames inputs.holochain.devShells;
      perSystem =
        { inputs'
        , config
        , pkgs
        , system
        , lib
        , self'
        , ...
        }: {
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              inputs'.tauri-plugin-holochain.devShells.holochainTauriDev 
              inputs'.hc-infra.devShells.synchronized-pnpm
              inputs'.holochain.devShells.holonix 
            ];
          };
          devShells.androidDev = pkgs.mkShell {
            inputsFrom = [
              inputs'.tauri-plugin-holochain.devShells.holochainTauriAndroidDev 
              inputs'.hc-infra.devShells.synchronized-pnpm
              inputs'.holochain.devShells.holonix 
            ];
          };
        };
    };
}
"#
        );
    }

    fn empty_package_json(package_name: &str) -> String {
        format!(
            r#"{{
  "name": "{package_name}",
  "dependencies": {{}}
}}
"#
        )
    }

    fn empty_web_happ_yaml(web_happ_name: &str) -> String {
        format!(
            r#"
---
manifest_version: "1"
name: {web_happ_name}
ui:
  bundled: "../ui/dist.zip"
happ_manifest:
  bundled: "./plenty.happ"
"#
        )
    }

    fn default_flake_nix() -> String {
        String::from(
            r#"{
  description = "Template for Holochain app development";
  
  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/weekly";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
    hc-infra.url = "github:holochain-open-dev/utils";
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
    {
      inherit inputs;
      specialArgs = {
        rootPath = ./.;
      };
    }
    {

      systems = builtins.attrNames inputs.holochain.devShells;
      perSystem =
        { inputs'
        , config
        , pkgs
        , system
        , lib
        , self'
        , ...
        }: {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ 
              inputs'.hc-infra.devShells.synchronized-pnpm
              inputs'.holochain.devShells.holonix 
            ];
          };
        };
    };
}
"#,
        )
    }
}
