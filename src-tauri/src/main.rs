#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// #[cfg(debug_assertions)]
// #[cfg(target_os = "macos")]
// embed_plist::embed_info_plist!("../Info.plist");

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
