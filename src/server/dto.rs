use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetValueRequestDto {
    pub key: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetValueResponseDto {
    pub value: String,
    pub checksum: String
}


#[derive(Serialize, Deserialize)]
pub struct GetChecksumResponseDto {
    pub checksum: String
}

#[derive(Serialize, Deserialize)]
pub struct PostValueRequestDto {
    pub key: String,
    pub value: String,
    pub namespace: String
}

#[derive(Serialize, Deserialize)]
pub struct DeleteValueRequestDto {
    pub key: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostNamespaceRequestDto {
    pub namespace: String
}

