use std::{env, error::Error, fs, path::PathBuf, str::FromStr};

use crate::storage::utils::{preprocess_filename, preprocess_namespace};

const DB_ROOT_KEY: &'static str = "DB_PATH";

fn get_root_directory() -> PathBuf {
    let stringified = env::var(DB_ROOT_KEY).unwrap_or("./data".to_string());
    return PathBuf::from_str(&stringified).unwrap();
}


fn resolve_file_path(namespace: String, file_name: String) -> PathBuf {
    let processed_namespace = preprocess_namespace(namespace);
    let processed_name = preprocess_filename(file_name);
    let mut file_path = get_root_directory();
    file_path.push(processed_namespace);
    file_path.push(processed_name);
    file_path
}

pub fn get_file_content(namespace: String, file_name: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_path = resolve_file_path(namespace, file_name);
    match fs::exists(file_path.clone()) {
        Err(_) => {
            return Err(Box::from(format!("Could not open file {:?}", file_path)))
        },
        Ok(val) => {
            if !val {
                return Err(Box::from("File does not exist"))
            }
        }
    };
    Ok(fs::read(file_path)?)
}

pub fn save_file_content(namespace: String, file_name: String, content: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let file_path = resolve_file_path(namespace, file_name);
    fs::write(file_path, content)?;
    Ok(())
}

pub fn create_namespace_directory(namespace: String) -> Result<(), Box<dyn Error>> {
    let mut path = get_root_directory();
    let directory_path = preprocess_namespace(namespace);
    path.push(directory_path);
    fs::create_dir_all(path)?;
    Ok(())
}
