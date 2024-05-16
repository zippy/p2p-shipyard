use anyhow::{anyhow, Result};
use build_fs_tree::{Build, MergeableFileSystemTree};
use clap::Parser;
use colored::Colorize;
use scaffold_holochain_runtime::scaffold_holochain_runtime;
use std::{
    ffi::OsString,
    fs,
    path::PathBuf,
    process::{Command, ExitCode},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the holochain runtime
    #[clap(long)]
    pub name: Option<String>,

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

    let (name, file_tree) = scaffold_holochain_runtime(args.name)?;

    let runtime_path = args.path.join(&name);

    if std::fs::canonicalize(&runtime_path)?.exists() {
        return Err(anyhow!(
            "The directory {runtime_path:?} already exists: choose another name"
        ));
    }

    fs::create_dir_all(&runtime_path)?;

    let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);

    file_tree.build(&runtime_path)?;

    println!(
        "{}",
        format!("Successfully scaffolded holochain runtime").green()
    );

    println!("Running nix flake update...");
    Command::new("nix")
        .args(["flake", "update"])
        .current_dir(runtime_path)
        .output()?;

    Ok(())
}
