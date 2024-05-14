use crate::HolochainExt;
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) fn open_app<R: Runtime>(
    app: AppHandle<R>,
    app_id: String,
    label: String,
    title: String,
    // url_path: Option<String>,
) -> crate::Result<()> {
    app.holochain()?
        .web_happ_window_builder(app_id)
        .label(label)
        .title(title)
        // .url_path(url_path)
        .build()
}
