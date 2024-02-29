use axum::{extract::State, http::StatusCode, Json};

use chrono::{DateTime, Utc};
use entity::category_book;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::utils::api_error::APIError;

#[derive(Deserialize)]
pub struct CategoryResquest {
    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
    id: i32,
    title: String,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

pub async fn create_category(
    State(database): State<DatabaseConnection>,

    Json(category): Json<CategoryResquest>,
) -> Result<Json<CategoryResponse>, APIError> {
    let category = category_book::ActiveModel {
        title: Set(category.title),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|err| {
        eprintln!("{:?}", err);
        APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;
    let created_at_utc = category.created_at.unwrap().with_timezone(&Utc);
    let updated_at_utc = category.updated_at.unwrap().with_timezone(&Utc);

    Ok(Json(CategoryResponse {
        id: category.id.unwrap(),
        title: category.title.unwrap(),
        created_at: created_at_utc,
        update_at: updated_at_utc,
    }))
}
