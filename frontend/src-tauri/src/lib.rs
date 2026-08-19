mod commands;
use commands::*;
use std::sync::Mutex;
use tauri::{Builder, Manager};
use Core::parser::*;
use std::fs::File;

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
        app.manage(Mutex::new(Option::None::<File>));
        app.manage(Mutex::new(vec![Window::default(String::from("none"))]));
        app.manage(Mutex::new(vec![vec![String::from("")]])); //stored components, UDPs, functions, databases, structs, resources
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_file_paths, create_file, delete_file, load_file, close_file, back, enter, 
      save_file, edit_field, add_dependency, add_x, remove_x, request_x])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}