use anyhow::Result;
use build_fs_tree::dir;
use dialoguer::{theme::ColorfulTheme, Input};
use file_tree_utils::{dir_to_file_tree, FileTree, FileTreeError};
use handlebars::RenderError;
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use templates_scaffolding_utils::{
    register_case_helpers, render_template_file_tree_and_merge_with_existing,
    TemplatesScaffoldingUtilsError,
};
use thiserror::Error;

static TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/holochain-runtime");

#[derive(Error, Debug)]
pub enum ScaffoldHolochainRuntimeError {
    #[error(transparent)]
    RenderError(#[from] RenderError),

    #[error(transparent)]
    TemplatesScaffoldingUtilsError(#[from] TemplatesScaffoldingUtilsError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    DialoguerError(#[from] dialoguer::Error),

    #[error(transparent)]
    FileTreeError(#[from] FileTreeError),
}

#[derive(Serialize, Deserialize, Debug)]
struct ScaffoldHolochainRuntimeData {
    name: String,
}

pub fn scaffold_holochain_runtime(
    name: Option<String>,
) -> Result<(String, FileTree), ScaffoldHolochainRuntimeError> {
    let name = match name {
        Some(name) => name,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a name for your holochain runtime (eg. launcher):")
            .interact_text()?,
    };

    // - Create the src-tauri directory structure
    let template_file_tree = dir_to_file_tree(&TEMPLATE)?;
    let h = handlebars::Handlebars::new();
    let h = register_case_helpers(h);

    let existing_file_tree = dir! {};

    let file_tree = render_template_file_tree_and_merge_with_existing(
        existing_file_tree,
        &h,
        &template_file_tree,
        &ScaffoldHolochainRuntimeData { name: name.clone() },
    )?;

    Ok((name, file_tree))
}
