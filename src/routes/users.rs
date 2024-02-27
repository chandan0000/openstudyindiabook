use crate::utils::{api_error::APIError, jwt::{decode_jwt, encode_jwt}};

use axum::{extract::State, http::StatusCode, Error, Json};
use bcrypt;
use entity::{user, user::Entity as User};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserRequest {
    name: String,
    email: String,
    password: String,
}
#[derive(Serialize, Clone)]
pub struct ResponseUser {
    id: i32,
    name: String,
    email: String,
    token: Option<String>,
}
pub async fn create_user(
    State(database): State<DatabaseConnection>,
    Json(user_request): Json<UserRequest>,
) -> Result<Json<ResponseUser>, APIError> {
    // Validate email
    if user_request.email.is_empty() {
        return Err(APIError {
            message: "Email cannot be empty".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    // Check if user with the same email already exists
    if let Some(existing_user) = User::find()
        .filter(user::Column::Email.eq(&user_request.email))
        .one(&database)
        .await
        .map_err(|err| {
            eprintln!("Error checking for existing user: {:?}", err);
            APIError {
                message: "Internal Server Error".to_owned(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?
    {
        return Err(APIError {
            message: "User already exists".to_owned(),
            status_code: StatusCode::CONFLICT,
        });
    }

    let password_hash = hash_password(user_request.password)?;
    let user = user::ActiveModel {
        name: Set(user_request.name),
        email: Set(user_request.email),
        password: Set(password_hash), // Store the hashed password
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|err| {
        eprintln!("Error creating user: {:?}", err);
        APIError {
            message: "Internal Server Error".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    // Generate a token (replace this with your token generation logic)
    let token = encode_jwt(user.id.clone().unwrap())?;

    Ok(Json(ResponseUser {
        token: Some(token),
        id: user.id.unwrap(),
        name: user.name.unwrap(),
        email: user.email.unwrap(),
    }))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}
pub async fn login_user(
    State(database): State<DatabaseConnection>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, APIError> {
    // Find the user by email
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&login_request.email))
        .one(&database)
        .await
        .map_err(|err| {
            eprintln!("Error finding user: {:?}", err);
            APIError {
                message: "Internal Server Error".to_owned(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?
        .ok_or(APIError {
            message: "User not found".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    // Verify password
    let password_matched = verify_password(login_request.password, &user.password)?;

    if !password_matched {
        return Err(APIError {
            message: "Invalid password".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    // Generate token (replace this with your token generation logic)
    let token = encode_jwt(user.id.clone())?;

    Ok(Json(LoginResponse { token }))
}

pub async fn logout_user(State(database): State<DatabaseConnection>) {}

fn hash_password(password: String) -> Result<String, APIError> {
    bcrypt::hash(password, 14).map_err(|_error| APIError {
        message: _error.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })
}

fn verify_password(password: String, hash: &str) -> Result<bool, APIError> {
    bcrypt::verify(password, hash).map_err(|_error| APIError {
        message: _error.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })
}
