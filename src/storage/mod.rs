use std::{error::Error, path::PathBuf};

mod filesystem;
mod maps;

const DATA_FILE_NAME: &'static str = "data.dpdb";

pub fn insert(namespace: String, key: String, val: String) -> Result<(), Box<dyn Error>> {
    let file_content = match filesystem::get_file_content(namespace.clone(), DATA_FILE_NAME.to_owned()) {
        Ok(v) => v,
        Err(_) => {
            Vec::new()
        }
    };
    let output = maps::set_value(file_content, key, val);
    filesystem::save_file_content(namespace, DATA_FILE_NAME.to_owned(), output)?;
    Ok(())
}

pub fn get(namespace: String, key: &str) -> Result<Option<String>, Box<dyn Error>> {
    let file_content = filesystem::get_file_content(namespace.clone(), DATA_FILE_NAME.to_owned())?;
    Ok(maps::get_value(file_content, key))
}

pub fn delete(namespace: PathBuf, key: &str) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn create_namespace(namespace: PathBuf) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn get_checksum() -> String {
    todo!()
}
