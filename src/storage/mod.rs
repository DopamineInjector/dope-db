use std::error::Error;

mod filesystem;
mod maps;
mod utils;
mod checksum;

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

pub fn delete(namespace: String, key: &str) -> Result<(), Box<dyn Error>> {
    let file_content = filesystem::get_file_content(namespace.clone(), DATA_FILE_NAME.to_owned())?;
    let output = maps::delete_value(file_content, key);
    filesystem::save_file_content(namespace, DATA_FILE_NAME.to_owned(), output)?;
    Ok(())
}

pub fn create_namespace(namespace: String) -> Result<(), Box<dyn Error>> {
    filesystem::create_namespace_directory(namespace)
}

pub fn get_checksum() -> String {
    todo!()
}
