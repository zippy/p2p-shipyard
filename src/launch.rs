use std::{sync::Arc, time::Duration};

use holochain_types::websocket::AllowedOrigins;

use app_dirs2::AppDataType;
use tokio::io::AsyncWriteExt;

use holochain::conductor::{state::AppInterfaceId, Conductor, ConductorHandle};
use holochain_client::{AdminWebsocket, AppWebsocket};
use holochain_keystore::{lair_keystore::spawn_lair_keystore, LairResult, MetaLairClient};
use lair_keystore::{
    dependencies::{
        lair_keystore_api::prelude::{LairServerConfig, LairServerConfigInner},
        sodoken::BufRead,
    },
    server::StandaloneServer,
};

use crate::{filesystem::FileSystem, HolochainRuntime};

fn override_gossip_arc_clamping() -> Option<String> {
    if cfg!(mobile) {
        Some(String::from("empty"))
    } else {
        None
    }
}

// pub static RUNNING_HOLOCHAIN: RwLock<Option<RunningHolochainInfo>> = RwLock::const_new(None);

pub async fn launch(passphrase: BufRead) -> crate::Result<HolochainRuntime> {
    // let mut lock = RUNNING_HOLOCHAIN.write().await;

    // if let Some(info) = lock.to_owned() {
    //     return Ok(info);
    // }

    let app_data_dir = app_dirs2::app_root(
        AppDataType::UserData,
        &app_dirs2::AppInfo {
            name: "tauri-plugin-holochain",
            author: "darksoil.studio",
        },
    )?
    .join("holochain");
    let app_config_dir = app_dirs2::app_root(
        AppDataType::UserConfig,
        &app_dirs2::AppInfo {
            name: "tauri-plugin-holochain",
            author: "darksoil.studio",
        },
    )?
    .join("holochain");

    let filesystem = FileSystem::new(app_data_dir, app_config_dir).await?;
    let admin_port = portpicker::pick_unused_port().expect("No ports free");
    let app_port = portpicker::pick_unused_port().expect("No ports free");

    let fs = filesystem.clone();

    // let config = get_config(&fs.keystore_config_path(), passphrase.clone())
    // .await
    // .map_err(|err| crate::Error::LairError(err))?;

    // TODO: this fails with "query returned no rows" error, fix it
    // let store_factory = lair_keystore::create_sql_pool_factory(&fs.keystore_store_path());

    // // create an in-process keystore with an in-memory store
    // let keystore = InProcKeystore::new(config.clone(), store_factory, passphrase.clone())
    //     .await
    //     .map_err(|err| crate::Error::LairError(err))?;

    // let client = keystore
    //     .new_client()
    //     .await
    //     .map_err(|err| crate::Error::LairError(err))?;

    // let lair_client = MetaLairClient::new_with_client(client.clone())
    //     .await
    //     .map_err(|err| crate::Error::LairError(err))?;

    // let lair_client = spawn_lair_keystore_in_proc(fs.keystore_config_path(), passphrase.clone())
    //     .await
    //     .map_err(|err| crate::Error::LairError(err))?;

    // let connection_url = config.connection_url.clone();
    // let connection_url = url2::Url2::parse("http://localhost:8990");

    // tauri::async_runtime::spawn(async move {
    let conductor_handle = build_conductor(&fs, admin_port, app_port, passphrase).await?;

    wait_until_admin_ws_is_available(admin_port).await?;
    log::info!("Connected to the admin websocket");

    wait_until_app_ws_is_available(app_port).await?;
    log::info!("Connected to the app websocket at port {app_port}");

    // *lock = Some(info.clone());

    Ok(HolochainRuntime {
        filesystem: fs,
        app_port,
        admin_port,
        conductor_handle,
    })
}

async fn build_conductor(
    fs: &FileSystem,
    admin_port: u16,
    app_port: u16,
    passphrase: BufRead,
) -> crate::Result<ConductorHandle> {
    let config = crate::config::conductor_config(
        &fs,
        admin_port,
        fs.keystore_dir().into(),
        override_gossip_arc_clamping(),
    );

    let conductor = Conductor::builder()
        .config(config)
        .passphrase(Some(passphrase))
        .build()
        .await?;

    let p: either::Either<u16, AppInterfaceId> = either::Either::Left(app_port);
    conductor
        .clone()
        .add_app_interface(p, AllowedOrigins::Any)
        .await?;

    Ok(conductor)
}

pub async fn wait_until_admin_ws_is_available(admin_port: u16) -> crate::Result<()> {
    let mut retry_count = 0;
    loop {
        if let Err(err) = AdminWebsocket::connect(format!("127.0.0.1:{}", admin_port)).await {
            log::error!("Could not connect to the admin interface: {}", err);
        } else {
            break;
        }
        async_std::task::sleep(Duration::from_millis(200)).await;

        retry_count += 1;
        if retry_count == 200 {
            return Err(crate::Error::AdminWebsocketError(
                "Can't connect to holochain".to_string(),
            ));
        }
    }
    Ok(())
}

pub async fn wait_until_app_ws_is_available(app_port: u16) -> crate::Result<()> {
    let mut retry_count = 0;
    let _admin_ws = loop {
        if let Ok(ws) = AppWebsocket::connect(format!("127.0.0.1:{}", app_port))
            .await
            .map_err(|err| {
                crate::Error::AdminWebsocketError(format!(
                    "Could not connect to the app interface: {}",
                    err
                ))
            })
        {
            break ws;
        }
        async_std::task::sleep(Duration::from_millis(200)).await;

        retry_count += 1;
        if retry_count == 200 {
            return Err(crate::Error::AdminWebsocketError(
                "Can't connect to holochain".to_string(),
            ));
        }
    };
    Ok(())
}

// fn read_config(config_path: &std::path::Path) -> crate::Result<LairServerConfig> {
//     let bytes = std::fs::read(config_path)?;

//     let config =
//         LairServerConfigInner::from_bytes(&bytes).map_err(|err| crate::Error::LairError(err))?;

//     if let Err(e) = std::fs::read(config.clone().pid_file) {
//         // Workaround xcode different containers
//         std::fs::remove_dir_all(config_path.parent().unwrap())?;
//         std::fs::create_dir_all(config_path.parent().unwrap())?;
//         return Err(e)?;
//     }

//     Ok(Arc::new(config))
// }

// /// Spawn an in-process keystore backed by lair_keystore.
// pub async fn spawn_lair_keystore_in_proc(
//     config_path: std::path::PathBuf,
//     passphrase: BufRead,
// ) -> LairResult<MetaLairClient> {
//     // return Ok(spawn_test_keystore().await?);

//     let config = get_config(&config_path, passphrase.clone()).await?;
//     let connection_url = config.connection_url.clone();

//     // rather than using the in-proc server directly,
//     // use the actual standalone server so we get the pid-checks, etc
//     let mut server = StandaloneServer::new(config).await?;

//     server.run(passphrase.clone()).await?; // 3 seconds

//     // just incase a Drop gets impld at some point...
//     std::mem::forget(server);

//     // now, just connect to it : )
//     let k = spawn_lair_keystore(connection_url.into(), passphrase).await?; // 2 seconds
//     Ok(k)
// }

// pub async fn get_config(
//     config_path: &std::path::Path,
//     passphrase: BufRead,
// ) -> LairResult<LairServerConfig> {
//     match read_config(config_path) {
//         Ok(config) => Ok(config),
//         Err(_) => write_config(config_path, passphrase).await,
//     }
// }

// pub async fn write_config(
//     config_path: &std::path::Path,
//     passphrase: BufRead,
// ) -> LairResult<LairServerConfig> {
//     let lair_root = config_path
//         .parent()
//         .ok_or_else(|| one_err::OneErr::from("InvalidLairConfigDir"))?;

//     tokio::fs::DirBuilder::new()
//         .recursive(true)
//         .create(&lair_root)
//         .await?;

//     let config = LairServerConfigInner::new(lair_root, passphrase).await?;

//     let mut config_f = tokio::fs::OpenOptions::new()
//         .write(true)
//         .create_new(true)
//         .open(config_path)
//         .await?;

//     config_f.write_all(config.to_string().as_bytes()).await?;
//     config_f.shutdown().await?;
//     drop(config_f);

//     Ok(Arc::new(config))
// }
