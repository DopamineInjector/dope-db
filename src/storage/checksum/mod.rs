use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChecksumChecker {
    namespace_hash: Vec<u8>,
    content_hash: Vec<u8>,
    children: HashMap<String, Box<ChecksumChecker>>,
}

fn deserialize(data: Vec<u8>) -> ChecksumChecker {
    serde_binary::from_vec(data, serde_binary::binary_stream::Endian::Little).unwrap()
}

fn serialize(data: &ChecksumChecker) -> Vec<u8> {
    serde_binary::to_vec(data, serde_binary::binary_stream::Endian::Little).unwrap()
}


