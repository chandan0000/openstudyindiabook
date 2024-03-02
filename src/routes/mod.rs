mod books;
mod category;
mod hello_server;
mod users;
use category::create_category;
use hello_server::hello_server;
use users::{create_user, login_user, logout_user};

use axum::{
    extract::FromRef,
    middleware,
    routing::{get, post},
    routing::{on, MethodFilter},
    Router,
};
use books::book_created;
use sea_orm::DatabaseConnection;

use crate::utils::guards::guard;

#[derive(Clone, FromRef)]

pub struct AppState {
    pub database: DatabaseConnection,
}

pub fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        .route("/logout", post(logout_user))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        //
        .route("/", get(hello_server))
        .route("/category", post(create_category))
        .route(
            "/books",
            post(book_created), // on(MethodFilter::POST, book_created)
        )
        .route("/users/signup", post(create_user))
        .route("/users/login", post(login_user))
        .with_state(app_state)
}
