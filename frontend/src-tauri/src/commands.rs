use Core::parser::*;
use Core::fileManager::*;
use directories::BaseDirs;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;
use tauri::{Builder, Manager};
use std::sync::Mutex;
use tauri::State;
use std::mem;

#[tauri::command]
pub fn enter(path_state: tauri::State<'_, Mutex<Vec<Window>>>, field_type: String, name: String) {
    let mut path_guard = path_state.lock().unwrap();
    let mut path = mem::replace(&mut *path_guard, vec![]);
    cd(name, field_type, &mut path);
    *path_guard = path;
}

#[tauri::command]
pub fn back(path_state: tauri::State<'_, Mutex<Vec<Window>>>) {
    let mut path_guard = path_state.lock().unwrap();
    let mut path = mem::replace(&mut *path_guard, vec![]);
    cd_back(&mut path);
    *path_guard = path;
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
            write(&newFile, base);
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
pub fn delete_file(path_state: tauri::State<'_, Mutex<Vec<Window>>>,
open_file: tauri::State<'_, Mutex<Option<File>>>, path: String) {
    let mut path_guard = path_state.lock().unwrap();
    let mut open_file_guard = open_file.lock().unwrap();
    *path_guard = vec![Window::default(String::from("none"))];
    *open_file_guard = Option::None;
    remove_path(&path);
    fs::remove_file(path);
}

#[tauri::command]
pub fn load_file(path_state: tauri::State<'_, Mutex<Vec<Window>>>, 
open_file: tauri::State<'_, Mutex<Option<File>>>, file_path: &str) {
    let mut path_guard = path_state.lock().unwrap();
    let mut open_file_guard = open_file.lock().unwrap();
    let file_path_buf = PathBuf::from(file_path);
    let file_promise = OpenOptions::new().read(true).write(true).open(file_path_buf);
    match file_promise {
        Ok(file) => {
            let project_promise = read(&file);
            *open_file_guard = Option::Some(file);
            *path_guard = vec![Window::prj(project_promise.package)];
        }
        Err(e) => {}
    }
}

#[tauri::command]
pub fn close_file(path_state: tauri::State<'_, Mutex<Vec<Window>>>, 
open_file: tauri::State<'_, Mutex<Option<File>>>) {
    let mut path_guard = path_state.lock().unwrap();
    let mut open_file_guard = open_file.lock().unwrap();
    *path_guard = vec![Window::default(String::from("none"))];
    *open_file_guard = Option::None;
}

#[tauri::command]
pub fn save_file(path_state: tauri::State<'_, Mutex<Vec<Window>>>, 
open_file: tauri::State<'_, Mutex<Option<File>>>) {
    let mut path_guard = path_state.lock().unwrap();
    let mut open_file_guard = open_file.lock().unwrap();
    let mut inner_data = mem::replace(&mut *open_file_guard, Option::None::<File>);
    match inner_data {
        Option::Some(file) => {
            save(&mut *path_guard, &file);
            inner_data = Option::Some(file);
        }
        Option::None => {}
    }
    *open_file_guard = inner_data;
}

#[tauri::command]
pub fn edit_field(path_state: tauri::State<'_, Mutex<Vec<Window>>>, fieldName: &str, val: String) {
    let mut path_guard = path_state.lock().unwrap();
    edit(fieldName, val, &mut path_guard);
}

#[tauri::command]
pub fn add_dependency() {

}

#[tauri::command]
pub fn add_x(path_state: tauri::State<'_, Mutex<Vec<Window>>>, field_name: &str, init_val: String) {
    let mut path_guard = path_state.lock().unwrap();
    add(&mut path_guard, field_name, init_val);
}

#[tauri::command]
pub fn remove_x(path_state: tauri::State<'_, Mutex<Vec<Window>>>, field_name: &str, ID: usize) {
    let mut path_guard = path_state.lock().unwrap();
    remove(&mut path_guard, field_name, ID);
}

#[tauri::command]
pub fn request_x() {
    
}