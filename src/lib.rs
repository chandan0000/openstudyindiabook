mod routes;
mod utils;
use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use routes::create_routes;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn run() {
    let database_url = (*utils::contstants::DATABASE_URL).clone();

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    // .sqlx_logging_level(log::LevelFilter::Info);

    let db: DatabaseConnection = Database::connect(opt).await.unwrap();
    // Initialize database connection

    // Run all pending migrations
    Migrator::up(&db, None).await.unwrap();

    let app = create_routes(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
