use actix_web::{Responder, HttpResponse};
use actix_web::web;
use crate::db::connection::get_db_connection;
use crate::models::detail_content::DetailContent;

pub async fn get_detail_content_by_id(path: web::Path<i64>) -> impl Responder {
    let content_id = path.into_inner(); 
    match get_db_connection().await {
        Ok(client) => {
            match DetailContent::get_detail_by_id(&client, content_id).await {
                Ok(Some(detail)) => {
                    HttpResponse::Ok().json(detail)  // Mengembalikan detail content dalam format JSON
                },
                Ok(None) => {
                    HttpResponse::NotFound().json(["Content not found"])  // Jika tidak ditemukan
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
