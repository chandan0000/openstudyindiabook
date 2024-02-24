use axum::{extract::State, http::StatusCode, Error, Json};
use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserRequest {
    name: String,
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct ResponseUser {
    id: i32,
    name: String,
    email: String,
}
pub async fn create_user(
    State(database): State<DatabaseConnection>,

    Json(user_request): Json<UserRequest>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = user::ActiveModel {
        name: Set(user_request.name),
        email: Set(user_request.email),
        password: Set(user_request.password),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|err| {
      println!("{err}");
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ResponseUser {
        id: user.id.unwrap(),
        name: user.name.unwrap(),
        email: user.email.unwrap(),
    }))
}

pub async fn login_user(State(databse): State<DatabaseConnection>) {
   println!("hrrrr");
}

pub async fn logout_user(State(database): State<DatabaseConnection>) {}
