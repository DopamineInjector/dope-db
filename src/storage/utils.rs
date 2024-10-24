use std::{path::{Component, PathBuf}, str::FromStr};


pub fn preprocess_namespace(namespace: String) -> PathBuf {
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

pub fn preprocess_filename(file_name: String) -> PathBuf {
    let file_path = PathBuf::from_str(&file_name).unwrap();
    let name = file_path.file_name().unwrap();
    PathBuf::from_str(name.to_str().unwrap()).unwrap()
}
