use tauri::{command, AppHandle, Manager, Runtime};

use crate::{HolochainExt, HolochainPlugin};

#[command]
pub(crate) fn is_holochain_ready<R: Runtime>(app_handle: AppHandle<R>) -> bool {
    app_handle.try_state::<HolochainPlugin<R>>().is_some()
}
