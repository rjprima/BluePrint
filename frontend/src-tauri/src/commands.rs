use Core::parser::*;
use Core::fileManager::*;
use directories::BaseDirs;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use tauri::{Builder, Manager};
use std::sync::Mutex;
use tauri::State;

pub struct curr_proj {
    pub val: Option<Project>
}

#[tauri::command]
pub fn enter() {

}

#[tauri::command]
pub fn back() {

}

#[tauri::command]
pub fn get_file_paths() -> Vec<String> {
    if let Some(path_file_start) =  BaseDirs::new() {
        let mut path_file = path_file_start.data_local_dir().to_path_buf();
        path_file.push("BluePrint");
        match fs::create_dir_all(&path_file) {
            Ok(x) => {print!("success")}
            Err(e) => {print!("error: {}",e)}
        }
        path_file = path_file.join("filepath.txt");
        let contents = fs::read_to_string(&path_file);
        match contents {
            Ok(mut value) => {
                let mut paths: Vec<String> = vec![];
                for path in value.lines() {
                    paths.push(String::from(path));
                }
                return paths;
            }
            Err(e) => {
                print!("error: {}", e);
                OpenOptions::new().read(true).append(true).create(true).open(path_file);
            }
        }
    }
    return vec![String::from("helloworld")];
}

#[tauri::command]
pub fn create_file(path:&str, project_name:&str)->bool {
    let base = Project {
        ver: String::from("1.0"), 
        name: String::from(project_name), 
        standards: vec![], 
        system_group: String::from(""), 
        components: vec![], 
        mains: vec![], 
        tools: vec![]};
    add_path(path);
    let mut newPath = PathBuf::from(path);
    let extension = &[project_name, ".txt"].concat();
    let ex = &[path, project_name, ".txt"].concat();
    newPath.push(extension);
    print!("\n {} should match {}\n", newPath.to_string_lossy(), ex);
    match OpenOptions::new().read(true).write(true).create_new(true).open(newPath) {
        Ok(newFile) => {
            write(newFile, base);
            print!("successfully created new project\n");
            return true;
        }
        Err(e) => {
            print!("{}\n", e);
            return false;
        }
    }
}

#[tauri::command]
pub fn load_file(state: tauri::State<'_, Mutex<Option<Project>>>) {
    let mut guard = state.lock().unwrap();
    
}

#[tauri::command]
pub fn save_file() {

}

#[tauri::command]
pub fn edit_field() {

}

#[tauri::command]
pub fn add_dependency() {

}

#[tauri::command]
pub fn add_x() {
    
}