use actix_web::{Responder, HttpResponse};
use crate::db::connection::get_db_connection;

pub async fn get_categories() -> impl Responder {
    match get_db_connection().await {
        Ok(client) => {
            let rows = client.query("SELECT id, name FROM categories", &[]).await;
            
            match rows {
                Ok(rows) => {
                    let categories: Vec<String> = rows.iter()
                        .map(|row| row.get::<_, String>(1))  // Mengambil nama kategori
                        .collect();
                    HttpResponse::Ok().json(categories)  // Mengembalikan data dalam format JSON
                },
                Err(e) => {
                    eprintln!("Error executing query: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
