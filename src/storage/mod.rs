use std::error::Error;

mod filesystem;
mod maps;
mod utils;
mod checksum;

const DATA_FILE_NAME: &'static str = "data.dpdb";
const CHECKSUM_FILE_NAME: &'static str = "checksum.dpdb";

pub fn insert(namespace: String, key: String, val: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_content = match filesystem::get_file_content(namespace.clone(), DATA_FILE_NAME.to_owned()) {
        Ok(v) => v,
        Err(_) => {
            Vec::new()
        }
    };
    let output = maps::set_value(file_content, key, val);
    filesystem::save_file_content(namespace.clone(), DATA_FILE_NAME.to_owned(), output.clone())?;
    let clean_namespace = utils::preprocess_namespace(namespace);
    // Should be write through, but i am too lazy to implement that
    let checksum_file_content = filesystem::get_file_content(String::new(), CHECKSUM_FILE_NAME.to_owned()).unwrap_or(Vec::new());
    let (out, checksum) = checksum::update_checksum(checksum_file_content, clean_namespace, &output);
    let _ = filesystem::save_file_content(String::new(), CHECKSUM_FILE_NAME.to_owned(), out);
    Ok(checksum)
}

pub fn get(namespace: String, key: &str) -> Result<Option<String>, Box<dyn Error>> {
    let file_content = filesystem::get_file_content(namespace.clone(), DATA_FILE_NAME.to_owned())?;
    Ok(maps::get_value(file_content, key))
}

pub fn delete(namespace: String, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_content = filesystem::get_file_content(namespace.clone(), DATA_FILE_NAME.to_owned())?;
    let output = maps::delete_value(file_content, key);
    filesystem::save_file_content(namespace.clone(), DATA_FILE_NAME.to_owned(), output.clone())?;
    let clean_namespace = utils::preprocess_namespace(namespace);
    // Should be write through, but i am too lazy to implement that
    let checksum_file_content = filesystem::get_file_content(String::new(), CHECKSUM_FILE_NAME.to_owned()).unwrap_or(Vec::new());
    let (out, checksum) = checksum::update_checksum(checksum_file_content, clean_namespace, &output);
    let _ = filesystem::save_file_content(String::new(), CHECKSUM_FILE_NAME.to_owned(), out);
    Ok(checksum)
}

pub fn create_namespace(namespace: String) -> Result<(), Box<dyn Error>> {
    filesystem::create_namespace_directory(namespace)
}

pub fn get_checksum() -> Vec<u8> {
    let checksum_file_content = filesystem::get_file_content(String::new(), CHECKSUM_FILE_NAME.to_owned()).unwrap_or(Vec::new());
    checksum::get_checksum(checksum_file_content)
}
