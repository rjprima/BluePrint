use directories::BaseDirs;
use std::fs::{self, OpenOptions};
use std::fs::File;
use std::io::Write;
use std::fs::read_to_string;

pub fn getPaths() -> Result<File, std::io::Error> {
    if let Some(path_file_start) =  BaseDirs::new() {
        let mut path_file = path_file_start.data_local_dir().to_path_buf();
        path_file.push("BluePrint");
        fs::create_dir_all(&path_file);
        path_file = path_file.join("filepath.txt");
        return OpenOptions::new().read(true).append(true).create(true).open(&path_file);
    }
    else {
        #[cfg(windows)]
        {
            return File::options().read(true).write(true).open("NUL");
        }
        #[cfg(not(windows))]
        {
            File::options().read(true).write(true).open("/dev/null")
        }
    }
}

pub fn add_path(path: &str) {
    let file_paths = getPaths();
    match file_paths {
        Ok(mut paths) => {
            writeln!(paths, "{}", path);
        }
        Err(e) => {print!("failed to find path file")}
    }
}

pub fn remove_path(path: &str) {
    let file_paths = getPaths();
    if let Some(path_file) =  BaseDirs::new() {
        let contents = fs::read_to_string(path_file.data_local_dir().join("filePaths.txt"));
        match contents {
            Ok(mut value) => {
                value = value.replace(path, "");
                
            }
            Err(e) => {print!("error: {}", e)}
        }
    }
}