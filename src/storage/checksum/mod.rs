use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
struct ChecksumChecker {
    pub namespace_hash: Vec<u8>,
    pub content_hash: Vec<u8>,
    pub children_hash: Vec<u8>,
    children: BTreeMap<String, Box<ChecksumChecker>>,
}

impl ChecksumChecker {

    pub fn new() -> Self {
        let mut res = ChecksumChecker { 
            namespace_hash: Vec::new(), 
            content_hash: Vec::new(), 
            children_hash: Vec::new(), 
            children: BTreeMap::new(),
        };
        res.content_hash = calculate_hash(vec![Vec::new().as_slice()]);
        res.children_hash = calculate_hash(vec![Vec::new().as_slice()]);
        res.calculate_namespace_hash();
        res
    }

    fn strip_oldest_dir(&self, path: &PathBuf) -> Option<(PathBuf, String)> {
        let mut piter = path.components();
        let ancestor = piter.next()?.as_os_str().to_str()?.to_owned();
        Some((piter.as_path().to_path_buf(), ancestor))
    }

    fn calculate_namespace_hash(&mut self) {
        self.namespace_hash = calculate_hash(vec![&self.children_hash, &self.content_hash]);
    }

    pub fn recalculate_hash(&mut self, path: PathBuf, content: &[u8]) -> Vec<u8> {
        match self.strip_oldest_dir(&path) {
            Some((child_path, child)) => {
                let child = self.children.entry(child).or_insert(Box::new(ChecksumChecker::new()));
                let _ = child.recalculate_hash(child_path, content);
                let child_hashes = self.children
                    .values()
                    .map(|child| {
                        child.namespace_hash.as_slice()
                    })
                    .collect::<Vec<&[u8]>>();
                self.children_hash = calculate_hash(child_hashes);
                self.calculate_namespace_hash();
                self.namespace_hash.clone()
            },
            None => {
                self.content_hash = calculate_hash(vec![content]);
                self.calculate_namespace_hash();
                self.namespace_hash.clone()
            }
        }
    }
}

fn deserialize(data: Vec<u8>) -> ChecksumChecker {
    if data.len() == 0 {
        return ChecksumChecker::new();
    }
    serde_binary::from_vec(data, serde_binary::binary_stream::Endian::Little).unwrap()
}

fn serialize(data: &ChecksumChecker) -> Vec<u8> {
    serde_binary::to_vec(data, serde_binary::binary_stream::Endian::Little).unwrap()
}

fn calculate_hash(data: Vec<&[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    for entry in data {
        hasher.update(entry);
    }
    hasher.finalize().to_vec()
}

pub fn update_checksum(serialized: Vec<u8>, path: PathBuf, content: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut root = deserialize(serialized);
    let hash = root.recalculate_hash(path, content);
    let out = serialize(&root);
    (out, hash)
}

pub fn get_checksum(serialized: Vec<u8>) -> Vec<u8> {
    let root = deserialize(serialized);
    root.namespace_hash
}
