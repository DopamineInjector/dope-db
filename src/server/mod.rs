use axum::{http::StatusCode, routing::{delete, get, post}, Json, Router};
use axum_macros::debug_handler;
use base64::prelude::*;

use crate::storage;

use self::dto::{DeleteValueRequestDto, GetChecksumResponseDto, GetValueRequestDto, GetValueResponseDto, PostValueRequestDto};

mod dto;

pub async fn get_router() -> axum::Router {
    Router::new()
        .route("/api/get", get(get_value))
        .route("/api/checksum", get(get_checksum))
        .route("/api/insert", post(post_value))
        .route("/api/delete", delete(delete_value))
}

#[debug_handler]
async fn get_value(
    Json(payload): Json<GetValueRequestDto>
) -> (StatusCode, Json<Option<GetValueResponseDto>>){
    match storage::get(payload.namespace, &payload.key) {
        Err(_) => {
            (StatusCode::BAD_REQUEST, Json(None))
        },
        Ok(value) => {
            if value.is_none() {
                (StatusCode::NOT_FOUND, Json(None))
            } else {
                let checksum = storage::get_checksum();
                let checksum = BASE64_STANDARD.encode(checksum);
                let res = GetValueResponseDto{
                    checksum,
                    value: value.unwrap(),
                };
                (StatusCode::OK, Json(Some(res)))
            }
        }
    }
}

#[debug_handler]
async fn get_checksum() -> (StatusCode, Json<GetChecksumResponseDto>) {
    let checksum = storage::get_checksum();
    let checksum = BASE64_STANDARD.encode(checksum);
    let res = GetChecksumResponseDto {
        checksum
    };
    (StatusCode::OK, Json(res))
}

#[debug_handler]
async fn post_value(
    Json(payload): Json<PostValueRequestDto>
) -> (StatusCode, Json<Option<GetChecksumResponseDto>>) {
    match storage::insert(payload.namespace, payload.key, payload.value) {
        Ok(checksum) => {
            let checksum = BASE64_STANDARD.encode(checksum);
            let res = GetChecksumResponseDto {
                checksum
            };
            (StatusCode::CREATED, Json(Some(res)))
        },
        Err(_) => {
            (StatusCode::BAD_REQUEST, Json(None))
        }
    }
}

#[debug_handler]
async fn delete_value(
    Json(payload): Json<DeleteValueRequestDto>
) -> (StatusCode, Json<Option<GetChecksumResponseDto>>) {
    match storage::delete(payload.namespace, &payload.key) {
        Ok(checksum) => {
            let checksum = BASE64_STANDARD.encode(checksum);
            let res = GetChecksumResponseDto {
                checksum
            };
            (StatusCode::OK, Json(Some(res)))
        },
        Err(_) => {
            (StatusCode::NOT_FOUND, Json(None))
        }
    }
}
