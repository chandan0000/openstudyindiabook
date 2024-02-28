use crate::utils;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::api_error::APIError;

#[derive(Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub iat: usize,
    pub user_id: i32,
}

pub fn encode_jwt(user_id: i32) -> Result<String, APIError> {
    let now = Utc::now();
    let expire = Duration::days(4);

    let claim = Cliams {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        user_id,
    };
    let secret = (*utils::contstants::TOKEN).clone();

    return encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_error| APIError {
        message: _error.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    });
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>, APIError> {
    let secret = (*utils::contstants::TOKEN).clone();
    let res: Result<TokenData<Cliams>, APIError> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_error| APIError {
        message: _error.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    });
    return res;
}
