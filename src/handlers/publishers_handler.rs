use actix_web::{web, Responder};
use tokio_postgres::Client;
use serde::{Deserialize};

use crate::helpers::response::generate_response;
use crate::models::publishers::{
    fetch_publishers_latest, 
    fetch_publishers_featured, 
    fetch_publishers_by_category,
    fetch_publishers_by_author, 
    fetch_publishers_by_search,
};

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,  
}

pub async fn get_publishers(client: web::Data<Client>) -> impl Responder {
    let success_message = "Successfully fetched publishers";
    let error_message = "Failed to fetch publishers";

    match fetch_publishers_latest(&client).await {
        Ok(publishers) => generate_response(&200, "success", success_message, Some(publishers)),
        Err(err) => {
            eprintln!("Error fetching publishers: {}", err);
            generate_response::<()>( &500,"error", error_message, None)  
        }
    }
}

pub async fn get_publishers_featured(client: web::Data<Client>) -> impl Responder {
    let success_message = "Successfully fetched publishers featured";
    let error_message = "Failed to fetch publishers featured";

    match fetch_publishers_featured(&client).await {
        Ok(publishers) => generate_response(&200, "success", success_message, Some(publishers)),
        Err(err) => {
            eprintln!("Error fetching publishers: {}", err);
            generate_response::<()>( &500,"error", error_message, None)  
        }
    }
}

pub async fn get_publishers_by_category(
    path: web::Path<i32>,
    client: web::Data<Client>,
) -> impl Responder {
    let success_message = "Successfully fetched publishers by category";
    let error_message = "Failed to fetch publishers by category";
    let category_id: i32 = path.into_inner();

    match fetch_publishers_by_category(client.get_ref(), category_id).await {
        Ok(publishers_by_category) => generate_response(&200, "success", success_message, Some(publishers_by_category)),
        Err(err) => {
            eprintln!("Error fetching publishers: {}", err);
            generate_response::<()>( &500,"error", error_message, None)
        }
    }
}

pub async fn get_publishers_by_author(
    path: web::Path<i32>,
    client: web::Data<Client>,
) -> impl Responder {
    let success_message = "Successfully fetched publishers by author";
    let error_message = "Failed to fetch publishers by author";
    let author_id: i32 = path.into_inner();

    match fetch_publishers_by_author(client.get_ref(), author_id).await {
        Ok(publishers_by_author) => generate_response(&200, "success", success_message, Some(publishers_by_author)),
        Err(err) => {
            eprintln!("Error fetching publishers: {}", err);
            generate_response::<()>( &500,"error", error_message, None)
        }
    }
}

pub async fn search_publishers(
    query: web::Query<SearchQuery>, 
    client: web::Data<Client>,
) -> impl Responder {
    let success_message = "Successfully fetched publishers";
    let error_message = "Failed to fetch publishers";
    let query_string = &query.query;  

    match fetch_publishers_by_search(client.get_ref(), query_string).await {
        Ok(publishers) => generate_response(&200, "success", success_message, Some(publishers)),
        Err(err) => {
            eprintln!("Error fetching publishers: {}", err);
            generate_response::<()>( &500,"error", error_message, None)
        }
    }
}
