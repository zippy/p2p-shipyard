use std::{collections::HashMap, path::PathBuf, sync::Arc};

use http_server::{pong_iframe, read_asset};
use lair_keystore_api::LairClient;
pub use launch::RunningHolochainInfo;
use serde::{Deserialize, Serialize};
use tauri::{
    http::response,
    ipc::CapabilityBuilder,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, WebviewWindow, WebviewWindowBuilder,
};

use holochain::prelude::{AppBundle, ExternIO, MembraneProof, NetworkSeed, RoleName};
use holochain_client::{
    AdminWebsocket, AppAgentWebsocket, AppInfo, AppWebsocket, ConductorApiError, LairAgentSigner,
    ZomeCallTarget,
};
use holochain_conductor_api::CellInfo;
use holochain_types::web_app::WebAppBundle;

mod commands;
mod config;
mod error;
mod filesystem;
mod http_server;
mod launch;

use commands::install_web_app::{
    install_app, install_web_app, update_app, update_web_app, UpdateAppError,
};
pub use error::{Error, Result};
use filesystem::FileSystem;
pub use launch::launch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HolochainRuntimeInfo {
    http_server_port: u16,
    app_port: u16,
    admin_port: u16,
}

/// Access to the push-notifications APIs.
pub struct HolochainPlugin<R: Runtime> {
    pub app_handle: AppHandle<R>,
    pub filesystem: FileSystem,
    pub runtime_info: HolochainRuntimeInfo,
    pub lair_client: LairClient,
}

impl<R: Runtime> HolochainPlugin<R> {
    pub fn open_app(
        &self,
        app_id: String,
        label: String,
        title: String,
        url_path: Option<String>,
    ) -> crate::Result<()> {
        log::info!("Opening app {}", app_id);

        let path = url_path.unwrap_or_default();

        let mut window_builder = WebviewWindowBuilder::new(
            &self.app_handle,
            label.clone(),
            tauri::WebviewUrl::External(url::Url::parse(
                format!(
                    "happ://{app_id}/{path}",
                    // self.runtime_info.http_server_port
                )
                .as_str(),
            )?),
        )
        .initialization_script(
            format!(
                r#"
            window['__HC_LAUNCHER_ENV__'] = {{
                APP_INTERFACE_PORT: {},
                INSTALLED_APP_ID: "{}",
            }};
        "#,
                self.runtime_info.app_port, app_id
            )
            .as_str(),
        )
        .initialization_script(include_str!("../packages/signer/dist/index.js"));

        self.app_handle.add_capability(
            CapabilityBuilder::new("test")
                .window(label)
                .permission("holochain:allow-sign-zome-call"),
        )?;

        #[cfg(desktop)]
        {
            window_builder = window_builder
                .min_inner_size(1000.0, 800.0)
                .title(title.clone());
        }
        let _window = window_builder.build()?;

        log::info!("Opened app {}", app_id);
        Ok(())
    }

    pub async fn admin_websocket(&self) -> crate::Result<AdminWebsocket> {
        let admin_ws =
            AdminWebsocket::connect(format!("127.0.0.1:{}", self.runtime_info.admin_port))
                .await
                .map_err(|err| crate::Error::WebsocketConnectionError(format!("{err:?}")))?;
        Ok(admin_ws)
    }

    pub async fn app_websocket(&self) -> crate::Result<AppWebsocket> {
        let app_ws = AppWebsocket::connect(format!("127.0.0.1:{}", self.runtime_info.app_port))
            .await
            .map_err(|err| crate::Error::WebsocketConnectionError(format!("{err:?}")))?;
        Ok(app_ws)
    }

    pub async fn app_agent_websocket(&self, app_id: String) -> crate::Result<AppAgentWebsocket> {
        let app_ws = AppAgentWebsocket::connect(
            format!("127.0.0.1:{}", self.runtime_info.app_port),
            app_id,
            Arc::new(Box::new(LairAgentSigner::new(Arc::new(
                self.lair_client.clone(),
            )))),
        )
        .await
        .map_err(|err| crate::Error::WebsocketConnectionError(format!("{err:?}")))?;

        Ok(app_ws)
    }

    pub async fn install_web_app(
        &self,
        app_id: String,
        web_app_bundle: WebAppBundle,
        membrane_proofs: HashMap<RoleName, MembraneProof>,
        network_seed: Option<NetworkSeed>,
    ) -> crate::Result<AppInfo> {
        let mut admin_ws = self.admin_websocket().await?;
        let app_info = install_web_app(
            &mut admin_ws,
            &self.filesystem,
            app_id.clone(),
            web_app_bundle,
            membrane_proofs,
            network_seed,
        )
        .await?;

        self.app_handle.emit("app-installed", app_id)?;

        Ok(app_info)
    }

    pub async fn install_app(
        &self,
        app_id: String,
        app_bundle: AppBundle,
        membrane_proofs: HashMap<RoleName, MembraneProof>,
        network_seed: Option<NetworkSeed>,
    ) -> crate::Result<AppInfo> {
        let mut admin_ws = self.admin_websocket().await?;
        let app_info = install_app(
            &mut admin_ws,
            app_id.clone(),
            app_bundle,
            membrane_proofs,
            network_seed,
        )
        .await?;

        self.app_handle.emit("app-installed", app_id)?;
        Ok(app_info)
    }

    pub async fn update_web_app(
        &self,
        app_id: String,
        web_app_bundle: WebAppBundle,
    ) -> std::result::Result<(), UpdateAppError> {
        let mut admin_ws = self
            .admin_websocket()
            .await
            .map_err(|err| UpdateAppError::WebsocketError)?;
        let _app_info = update_web_app(
            &mut admin_ws,
            &self.filesystem,
            app_id.clone(),
            web_app_bundle,
        )
        .await?;

        self.app_handle.emit("app-updated", app_id)?;

        Ok(())
    }

    pub async fn update_app(
        &self,
        app_id: String,
        app_bundle: AppBundle,
    ) -> std::result::Result<(), UpdateAppError> {
        let mut admin_ws = self
            .admin_websocket()
            .await
            .map_err(|err| UpdateAppError::WebsocketError)?;
        let app_info = update_app(&mut admin_ws, app_id.clone(), app_bundle).await?;

        self.app_handle.emit("app-updated", app_id)?;
        Ok(app_info)
    }
}

// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the holochain APIs.
pub trait HolochainExt<R: Runtime> {
    fn holochain(&self) -> crate::Result<&HolochainPlugin<R>>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainExt<R> for T {
    fn holochain(&self) -> crate::Result<&HolochainPlugin<R>> {
        let s = self
            .try_state::<HolochainPlugin<R>>()
            .ok_or(crate::Error::HolochainNotInitialized)?;

        Ok(s.inner())
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>(subfolder: PathBuf) -> TauriPlugin<R> {
    Builder::new("holochain")
        .invoke_handler(tauri::generate_handler![
            commands::sign_zome_call::sign_zome_call,
            commands::get_locales::get_locales,
            commands::open_app::open_app,
            commands::list_apps::list_apps,
            commands::get_runtime_info::get_runtime_info,
            commands::get_runtime_info::is_holochain_ready
        ])
        .register_uri_scheme_protocol("happ", |app_handle, request| {
            log::info!("Received request {}", request.uri().to_string());
            if request.uri().to_string().starts_with("happ://ping") {
                return response::Builder::new()
                    .status(tauri::http::StatusCode::ACCEPTED)
                    .header("Content-Type", "text/html;charset=utf-8")
                    .body(pong_iframe().as_bytes().to_vec())
                    .expect("Failed to build body of accepted response");
            }
            // prepare our response
            tauri::async_runtime::block_on(async move {
                // let mutex = app_handle.state::<Mutex<AdminWebsocket>>();
                // let mut admin_ws = mutex.lock().await;

                let uri_without_protocol = request
                    .uri()
                    .to_string()
                    .split("://")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .get(1)
                    .expect("Malformed request: not enough items")
                    .clone();
                let uri_without_querystring: String = uri_without_protocol
                    .split("?")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .get(0)
                    .expect("Malformed request: not enough items 2")
                    .clone();
                let uri_components: Vec<String> = uri_without_querystring
                    .split("/")
                    .map(|s| s.to_string())
                    .collect();
                let lowercase_app_id = uri_components
                    .get(0)
                    .expect("Malformed request: not enough items 3");
                let mut asset_file = PathBuf::new();
                for i in 1..uri_components.len() {
                    asset_file = asset_file.join(uri_components[i].clone());
                }

                let Ok(holochain) = app_handle.holochain() else {
                    return response::Builder::new()
                        .status(tauri::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(
                            format!("Called http UI before initializing holochain")
                                .as_bytes()
                                .to_vec(),
                        )
                        .expect("Failed to build asset with not internal server error");
                };

                let r = match read_asset(
                    &holochain.filesystem,
                    lowercase_app_id,
                    asset_file
                        .as_os_str()
                        .to_str()
                        .expect("Malformed request: not enough items 4")
                        .to_string(),
                )
                .await
                {
                    Some((asset, mime_type)) => {
                        log::info!("Got asset for app with id: {}", lowercase_app_id);
                        let mut response =
                            response::Builder::new().status(tauri::http::StatusCode::ACCEPTED);
                        if let Some(mime_type) = mime_type {
                            response = response
                                .header("Content-Type", format!("{};charset=utf-8", mime_type))
                        } else {
                            response = response.header("Content-Type", "charset=utf-8")
                        }

                        return response
                            .body(asset)
                            .expect("Failed to build response with asset");
                    }
                    None => response::Builder::new()
                        .status(tauri::http::StatusCode::NOT_FOUND)
                        .body(vec![])
                        .expect("Failed to build asset with not found"),
                };

                // admin_ws.close();
                r
            })
        })
        .setup(|app, _api| {
            let handle = app.clone();
            let result =
                tauri::async_runtime::block_on(
                    async move { launch_and_setup_holochain(handle).await },
                );

            Ok(result?)
        })
        .build()
}

async fn launch_and_setup_holochain<R: Runtime>(app_handle: AppHandle<R>) -> crate::Result<()> {
    // let app_data_dir = app.path().app_data_dir()?.join(&subfolder);
    // let app_config_dir = app.path().app_config_dir()?.join(&subfolder);

    let http_server_port = portpicker::pick_unused_port().expect("No ports free");

    let RunningHolochainInfo {
        admin_port,
        app_port,
        lair_client,
        filesystem,
    } = launch().await?;

    log::info!("Starting http server at port {http_server_port:?}");

    http_server::start_http_server(app_handle.clone(), http_server_port).await?;

    let p = HolochainPlugin::<R> {
        app_handle: app_handle.clone(),
        lair_client,
        runtime_info: HolochainRuntimeInfo {
            http_server_port,
            app_port,
            admin_port,
        },
        filesystem,
    };

    // manage state so it is accessible by the commands
    app_handle.manage(p);

    app_handle.emit("holochain-ready", ())?;

    Ok(())
}
