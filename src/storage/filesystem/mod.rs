use std::{env, error::Error, fs, path::{Component, PathBuf}, str::FromStr};

const DB_ROOT_KEY: &'static str = "DB_PATH";

fn get_root_directory() -> PathBuf {
    let stringified = env::var(DB_ROOT_KEY).unwrap_or("./data".to_string());
    return PathBuf::from_str(&stringified).unwrap();
}

fn preprocess_namespace(namespace: String) -> PathBuf {
    let mut res = PathBuf::new();
    let namespace = PathBuf::from_str(&namespace).unwrap();
    // Might do something more fancy later, for now just strip all weird path elements
    // (we all know we will not, but it is nice to dream sometimes)
    namespace.components()
        .for_each(|component| {
            match component {
                Component::Normal(_) => {
                    res.push(component)
                }
                _ => {}
            }
        });
    res
}

fn preprocess_filename(file_name: String) -> PathBuf {
    let file_path = PathBuf::from_str(&file_name).unwrap();
    let name = file_path.file_name().unwrap();
    PathBuf::from_str(name.to_str().unwrap()).unwrap()
}

fn resolve_file_path(namespace: String, file_name: String) -> PathBuf {
    let processed_namespace = preprocess_namespace(namespace);
    let processed_name = preprocess_filename(file_name);
    let mut file_path = get_root_directory();
    file_path.push(processed_namespace);
    file_path.push(processed_name);
    dbg!(file_path)
}

pub fn get_file_content(namespace: String, file_name: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_path = resolve_file_path(namespace, file_name);
    match fs::try_exists(file_path.clone()) {
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
