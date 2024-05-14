use anyhow::Result;
use build_fs_tree::{Build, MergeableFileSystemTree};
use clap::Parser;
use colored::Colorize;
use scaffold_executable_happ::scaffold_executable_happ;
use std::{
    ffi::OsString,
    path::PathBuf,
    process::{Command, ExitCode},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the NPM package which contains the UI for this tauri app
    #[clap(long)]
    pub ui_package: Option<String>,

    /// The path of the file tree to modify.
    #[clap(long, default_value = "./.")]
    pub path: PathBuf,
}

fn main() -> ExitCode {
    if let Err(err) = internal_main() {
        eprintln!("{}", format!("Error: {err:?}").red());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn internal_main() -> Result<()> {
    let args = Args::parse();

    let file_tree = file_tree_utils::load_directory_into_memory(&args.path)?;

    let file_tree = scaffold_executable_happ(file_tree, args.ui_package)?;

    let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);

    file_tree.build(&args.path)?;

    println!(
        "{}",
        format!("Successfully scaffolded executable happ").green()
    );

    println!("Running nix flake update...");
    Command::new("nix").args(["flake", "update"]).output()?;

    // TODO: run {package_manager} install?

    Ok(())
}
