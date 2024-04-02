use holochain_types::web_app::WebAppBundle;
use lair_keystore::dependencies::sodoken::{BufRead, BufWrite};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri_plugin_holochain::HolochainExt;

pub fn example_happ() -> WebAppBundle {
    let bytes = include_bytes!("../../workdir/forum.webhapp");
    WebAppBundle::decode(bytes).expect("Failed to decode example webhapp")
}

pub fn vec_to_locked(mut pass_tmp: Vec<u8>) -> std::io::Result<BufRead> {
    match BufWrite::new_mem_locked(pass_tmp.len()) {
        Err(e) => {
            pass_tmp.fill(0);
            Err(e.into())
        }
        Ok(p) => {
            {
                let mut lock = p.write_lock();
                lock.copy_from_slice(&pass_tmp);
                pass_tmp.fill(0);
            }
            Ok(p.to_read())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_holochain::init(
            vec_to_locked(vec![]).expect("Can't build passphrase"),
        ))
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

            app.holochain()?
                .web_happ_window_builder(String::from("example"))
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
