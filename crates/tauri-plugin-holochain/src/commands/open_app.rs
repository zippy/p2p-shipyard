use crate::HolochainExt;
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) fn open_app<R: Runtime>(
    app: AppHandle<R>,
    app_id: String,
    title: String,
    url_path: Option<String>,
) -> crate::Result<()> {
    #[cfg(mobile)]
    {
        app.holochain()?.web_happ_window_builder(app_id, url_path)?.build()?;
    }

    #[cfg(desktop)]
    {
        app.holochain()?.web_happ_window_builder(app_id, url_path)?.title(title).build()?;
    }

    Ok(())
}
