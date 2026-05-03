mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;
use dotenv::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::connect_db().await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations Failed!");
    let app = routes::create_routes().with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Cozzy APIs is running on : {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
