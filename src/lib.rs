use std::{collections::HashMap, path::PathBuf, sync::Arc};

use hc_seed_bundle::dependencies::sodoken::BufRead;
use http_server::{pong_iframe, read_asset};
use lair_keystore_api::LairClient;
use serde::{Deserialize, Serialize};
use tauri::{
    http::response,
    ipc::CapabilityBuilder,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, WebviewWindow, WebviewWindowBuilder,
};

use holochain::{
    conductor::ConductorHandle,
    prelude::{AppBundle, MembraneProof, NetworkSeed, RoleName},
};
use holochain_client::{AdminWebsocket, AppAgentWebsocket, AppInfo, AppWebsocket, LairAgentSigner};
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

use crate::launch::wait_until_app_ws_is_available;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct HolochainRuntimeInfo {
//     http_server_port: u16,
//     app_port: u16,
//     admin_port: u16,
// }

/// Access to the push-notifications APIs.
pub struct HolochainPlugin<R: Runtime> {
    pub app_handle: AppHandle<R>,
    pub holochain_runtime: HolochainRuntime,
}

pub struct HolochainRuntime {
    pub filesystem: FileSystem,
    pub app_port: u16,
    pub admin_port: u16,
    pub(crate) conductor_handle: ConductorHandle,
}

pub struct WebHappWindowBuilder<R: Runtime> {
    app_handle: AppHandle<R>,
    app_port: u16,

    app_id: String,
    label: Option<String>,
    title: Option<String>,
    url_path: Option<String>,
}

impl<R: Runtime> WebHappWindowBuilder<R> {
    fn new(app_handle: AppHandle<R>, app_port: u16, app_id: String) -> Self {
        WebHappWindowBuilder {
            app_handle,
            app_port,
            app_id,
            label: None,
            title: None,
            url_path: None,
        }
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn url_path(mut self, url_path: String) -> Self {
        self.url_path = Some(url_path);
        self
    }

    pub fn build(self) -> crate::Result<()> {
        let label = self.label.unwrap_or(self.app_id.clone());
        let title = self.title.unwrap_or(label.clone());

        let url_path = self.url_path.unwrap_or_default();

        log::info!("Opening app {}", self.app_id);

        let url_origin = format!("happ://{}", self.app_id);

        let mut window_builder = WebviewWindowBuilder::new(
            &self.app_handle,
            label.clone(),
            tauri::WebviewUrl::External(url::Url::parse(
                format!("{url_origin}/{url_path}").as_str(),
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
                self.app_port, self.app_id
            )
            .as_str(),
        )
        .initialization_script(include_str!("../packages/signer/dist/index.js"));

        #[cfg(desktop)]
        {
            window_builder = window_builder
                .min_inner_size(1000.0, 800.0)
                .title(title.clone());
        }
        let _window = window_builder.build()?;

        let mut capability_builder =
            CapabilityBuilder::new("sign-zome-call").permission("holochain:allow-sign-zome-call");

        #[cfg(desktop)] // TODO: remove this check
        {
            capability_builder = capability_builder.window(label);
        }
        #[cfg(mobile)] // TODO: remove this check
        {
            capability_builder = capability_builder.windows(["*"]);
        }

        self.app_handle.add_capability(capability_builder)?;

        log::info!("Opened app {}", self.app_id);

        Ok(())
    }
}

impl<R: Runtime> HolochainPlugin<R> {
    pub fn web_happ_window_builder(&self, app_id: String) -> WebHappWindowBuilder<R> {
        WebHappWindowBuilder::new(
            self.app_handle.clone(),
            self.holochain_runtime.app_port.clone(),
            app_id,
        )
    }

    pub async fn admin_websocket(&self) -> crate::Result<AdminWebsocket> {
        let admin_ws =
            AdminWebsocket::connect(format!("127.0.0.1:{}", self.holochain_runtime.admin_port))
                .await
                .map_err(|err| crate::Error::WebsocketConnectionError(format!("{err:?}")))?;
        Ok(admin_ws)
    }

    pub async fn app_websocket(&self) -> crate::Result<AppWebsocket> {
        let app_ws =
            AppWebsocket::connect(format!("127.0.0.1:{}", self.holochain_runtime.app_port))
                .await
                .map_err(|err| crate::Error::WebsocketConnectionError(format!("{err:?}")))?;
        Ok(app_ws)
    }

    pub async fn app_agent_websocket(&self, app_id: String) -> crate::Result<AppAgentWebsocket> {
        let app_ws = AppAgentWebsocket::connect(
            format!("127.0.0.1:{}", self.holochain_runtime.app_port),
            app_id,
            Arc::new(Box::new(LairAgentSigner::new(Arc::new(
                self.holochain_runtime
                    .conductor_handle
                    .keystore()
                    .lair_client()
                    .clone(),
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
            &self.holochain_runtime.filesystem,
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
            &self.holochain_runtime.filesystem,
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
pub fn init<R: Runtime>(passphrase: BufRead) -> TauriPlugin<R> {
    Builder::new("holochain")
        .invoke_handler(tauri::generate_handler![
            commands::sign_zome_call::sign_zome_call,
            commands::get_locales::get_locales,
            commands::open_app::open_app,
            commands::list_apps::list_apps,
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

                let Ok(holochain_plugin) = app_handle.holochain() else {
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
                    &holochain_plugin.holochain_runtime.filesystem,
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
            let result = tauri::async_runtime::block_on(async move {
                launch_and_setup_holochain(handle, passphrase).await
            });

            Ok(result?)
        })
        .build()
}

async fn launch_and_setup_holochain<R: Runtime>(
    app_handle: AppHandle<R>,
    passphrase: BufRead,
) -> crate::Result<()> {
    // let app_data_dir = app.path().app_data_dir()?.join(&subfolder);
    // let app_config_dir = app.path().app_config_dir()?.join(&subfolder);

    // let http_server_port = portpicker::pick_unused_port().expect("No ports free");

    // http_server::start_http_server(app_handle.clone(), http_server_port).await?;
    // log::info!("Starting http server at port {http_server_port:?}");

    let holochain_runtime = launch(passphrase).await?;

    let p = HolochainPlugin::<R> {
        app_handle: app_handle.clone(),
        holochain_runtime,
    };

    // manage state so it is accessible by the commands
    app_handle.manage(p);

    app_handle.emit("holochain-ready", ())?;

    Ok(())
}
