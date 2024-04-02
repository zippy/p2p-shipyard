use std::path::PathBuf;
use std::{fs, io::Write};

use holochain::prelude::*;
use holochain_types::web_app::WebAppBundle;
use mr_bundle::error::MrBundleError;
use zip::result::ZipError;

#[derive(Clone)]
pub struct FileSystem {
    pub app_data_dir: PathBuf,
    pub app_config_dir: PathBuf,
}

impl FileSystem {
    pub async fn new(app_data_dir: PathBuf, app_config_dir: PathBuf) -> crate::Result<FileSystem> {
        let fs = FileSystem {
            app_data_dir,
            app_config_dir,
        };

        fs::create_dir_all(fs.webapp_store().path)?;
        fs::create_dir_all(fs.icon_store().path)?;
        fs::create_dir_all(fs.ui_store().path)?;
        fs::create_dir_all(fs.keystore_dir())?;

        Ok(fs)
    }

    pub fn keystore_dir(&self) -> PathBuf {
        self.app_data_dir.join("keystore")
    }

    pub fn keystore_config_path(&self) -> PathBuf {
        self.keystore_dir().join("lair-keystore-config.yaml")
    }

    pub fn keystore_store_path(&self) -> PathBuf {
        self.keystore_dir().join("store_file")
    }

    pub fn conductor_dir(&self) -> PathBuf {
        self.app_data_dir.join("conductor")
    }

    pub fn webapp_store(&self) -> WebAppStore {
        WebAppStore {
            path: self.app_data_dir.join("webhapps"),
        }
    }

    pub fn icon_store(&self) -> IconStore {
        IconStore {
            path: self.app_data_dir.join("icons"),
        }
    }

    pub fn ui_store(&self) -> UiStore {
        UiStore {
            path: self.app_data_dir.join("uis"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error(transparent)]
    MrBundleError(#[from] MrBundleError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ZipError(#[from] ZipError),
}

pub struct UiStore {
    path: PathBuf,
}

impl UiStore {
    pub fn ui_path(&self, installed_app_id: &InstalledAppId) -> PathBuf {
        self.path.join(installed_app_id)
    }

    pub async fn extract_and_store_ui(
        &self,
        installed_app_id: &InstalledAppId,
        web_app: &WebAppBundle,
    ) -> Result<(), FileSystemError> {
        let ui_bytes = web_app.web_ui_zip_bytes().await?;

        let ui_folder_path = self.ui_path(installed_app_id);

        if ui_folder_path.exists() {
            fs::remove_dir_all(&ui_folder_path)?;
        }

        fs::create_dir_all(&ui_folder_path)?;

        let ui_zip_path = self.path.join("ui.zip");

        fs::write(ui_zip_path.clone(), ui_bytes.into_owned().into_inner())?;

        let file = std::fs::File::open(ui_zip_path.clone())?;
        unzip_file(file, ui_folder_path)?;

        fs::remove_file(ui_zip_path)?;

        Ok(())
    }
}

pub struct WebAppStore {
    path: PathBuf,
}

impl WebAppStore {
    fn webhapp_path(&self, web_app_entry_hash: &EntryHash) -> PathBuf {
        let web_app_entry_hash_b64 = EntryHashB64::from(web_app_entry_hash.clone()).to_string();
        self.path.join(web_app_entry_hash_b64)
    }

    pub fn webhapp_package_path(&self, web_app_entry_hash: &EntryHash) -> PathBuf {
        self.webhapp_path(web_app_entry_hash)
            .join("package.webhapp")
    }

    pub fn get_webapp(
        &self,
        web_app_entry_hash: &EntryHash,
    ) -> crate::Result<Option<WebAppBundle>> {
        let path = self.webhapp_path(web_app_entry_hash);

        if path.exists() {
            let bytes = fs::read(self.webhapp_package_path(&web_app_entry_hash))?;
            let web_app = WebAppBundle::decode(bytes.as_slice())?;

            return Ok(Some(web_app));
        } else {
            return Ok(None);
        }
    }

    pub async fn store_webapp(
        &self,
        web_app_entry_hash: &EntryHash,
        web_app: &WebAppBundle,
    ) -> crate::Result<()> {
        let bytes = web_app.encode()?;

        let path = self.webhapp_path(web_app_entry_hash);

        fs::create_dir_all(path.clone())?;

        let mut file = std::fs::File::create(self.webhapp_package_path(web_app_entry_hash))?;
        file.write_all(bytes.as_slice())?;

        Ok(())
    }
}

pub struct IconStore {
    path: PathBuf,
}

impl IconStore {
    fn icon_path(&self, app_entry_hash: &ActionHash) -> PathBuf {
        self.path
            .join(ActionHashB64::from(app_entry_hash.clone()).to_string())
    }

    pub fn store_icon(&self, app_entry_hash: &ActionHash, icon_src: String) -> crate::Result<()> {
        fs::write(self.icon_path(app_entry_hash), icon_src.as_bytes())?;

        Ok(())
    }

    pub fn get_icon(&self, app_entry_hash: &ActionHash) -> crate::Result<Option<String>> {
        let icon_path = self.icon_path(app_entry_hash);
        if icon_path.exists() {
            let icon = fs::read_to_string(icon_path)?;
            return Ok(Some(icon));
        } else {
            return Ok(None);
        }
    }
}

pub fn unzip_file(reader: std::fs::File, outpath: PathBuf) -> Result<(), FileSystemError> {
    let mut archive = zip::ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to archive by index");
        let outpath = match file.enclosed_name() {
            Some(path) => outpath.join(path).to_owned(),
            None => continue,
        };

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
