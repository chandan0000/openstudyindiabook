
mod routes;
mod utils;
mod socket;
use migration::{Migrator, MigratorTrait};
use routes::create_routes;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use socketioxide::SocketIo;
use std::{env, time::Duration};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use socket::socket_manager::on_connect;


pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
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
    tracing::subscriber::set_global_default(FmtSubscriber::default());
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    io.ns("/custom", on_connect);


    

    // Initialize database connection

    // Run all pending migrations
    // Migrator::up(&db, None).await.unwrap();

    let app = create_routes(db).layer(layer);
    info!("Starting server");

    let listener = tokio::net::TcpListener::bind(env::var("localhost").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
