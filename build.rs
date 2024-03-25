const COMMANDS: &[&str] = &[
    "sign_zome_call",
    "get_locales",
    "open_app",
    "list_apps",
    "get_runtime_info",
    "is_holochain_ready",
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
