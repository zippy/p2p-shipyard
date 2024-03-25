use holochain_types::web_app::WebAppBundle;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri_plugin_holochain::HolochainExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn example_happ() -> WebAppBundle {
    let bytes = include_bytes!("../../workdir/forum.webhapp");
    WebAppBundle::decode(bytes).expect("Failed to decode example webhapp")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Error)
                // .clear_targets()
                // .target(Target::new(TargetKind::LogDir { file_name: None }))
                .build(),
        )
        .plugin(tauri_plugin_holochain::init(PathBuf::from("holochain")))
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let mut admin_ws = handle.holochain()?.admin_websocket().await?;

                let installed_apps = admin_ws
                    .list_apps(None)
                    .await
                    .map_err(|err| tauri_plugin_holochain::Error::ConductorApiError(err))?;

                if installed_apps.len() == 0 {
                    handle
                        .holochain()?
                        .install_web_app(
                            String::from("example"),
                            example_happ(),
                            HashMap::new(),
                            None,
                        )
                        .await
                        .map(|_| ())
                } else {
                    Ok(())
                }
            })?;

            app.holochain()?.open_app(
                String::from("example"),
                String::from("example"),
                String::from("Example"),
                None,
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
