use tokio_postgres::{Client, NoTls};
use std::env;
use dotenv::dotenv;

pub async fn get_db_connection() -> Result<Client, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}
