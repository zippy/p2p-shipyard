use crate::HolochainExt;
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) fn open_app<R: Runtime>(
    app: AppHandle<R>,
    app_id: String,
    title: String,
    url_path: Option<String>,
) -> crate::Result<()> {
    let mut builder = app.holochain()?.web_happ_window_builder(app_id, url_path)?;

    #[cfg(desktop)]
    {
        builder = builder.title(title);
    }

    builder.build()?;

    Ok(())
}
