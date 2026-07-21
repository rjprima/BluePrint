mod commands;
use commands::*;
use std::sync::Mutex;
use tauri::{Builder, Manager};
use Core::parser::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
        app.manage(Mutex::new(curr_proj {val: None}));
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_file_paths, create_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}