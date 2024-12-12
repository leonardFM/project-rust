use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

use crate::routes::home_routes::home_routes;
use crate::routes::category_routes::category_routes;
use crate::routes::detail_content_routes::detail_content_routes;
use crate::routes::publishers_routes::publishers_routes;

mod routes;
mod handlers;
mod config;
mod db;
mod models;
mod helpers;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "code": 200,
        "message": "Welcome to the API rust with actix-web VOI.ID"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Connect to PostgreSQL database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.expect("Failed to connect to database");

    // Spawn the database connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    // Wrap the client in web::Data
    let client_data = web::Data::new(client);

    // Get server address from .env or use default
    let server_address = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:7000".to_string());

    println!("Starting server at: {}", server_address);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(client_data.clone()) // Inject database client
            .configure(home_routes)       // Register routes
            .configure(category_routes)
            .configure(detail_content_routes)
            .configure(publishers_routes)
            .service(index)
    })
    .bind(&server_address)
    .map_err(|e| {
        eprintln!("Failed to bind server: {}", e);
        e
    })?
    .run()
    .await
}
