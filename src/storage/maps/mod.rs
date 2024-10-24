use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

// Potential improvement would be to cache some data in memory. However, that would have to be
// implemented on some upper level function

#[derive(Serialize, Deserialize)]
struct NamespaceData {
    content: BTreeMap<String, String>,
}

impl NamespaceData {
    pub fn new() -> Self {
        NamespaceData { 
            content: BTreeMap::new(), 
        }
    }
}

fn deserialize(byte_data: Vec<u8>) -> NamespaceData {
    if byte_data.len() == 0 {
        return NamespaceData::new();
    }
    serde_binary::from_vec(byte_data, serde_binary::binary_stream::Endian::Little).unwrap()
}

fn serialize(data: &NamespaceData) -> Vec<u8> {
    serde_binary::to_vec(data, serde_binary::binary_stream::Endian::Little).unwrap()
}

pub fn get_value(byte_data: Vec<u8>, key: &str) -> Option<String> {
    let data = deserialize(byte_data);
    data.content.get(key).cloned()
}

pub fn set_value(byte_data: Vec<u8>, key: String, value: String) -> Vec<u8> {
    let mut data = deserialize(byte_data);
    data.content.insert(key, value);
    serialize(&data)
}
