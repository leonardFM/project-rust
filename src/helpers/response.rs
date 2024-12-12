use actix_web::{HttpResponse};
use serde::{Serialize};
use serde_json::json;

#[derive(Serialize)]
pub struct ResponseData<T> {
    pub data: Option<T>,
}

pub fn generate_response<T>(code: &i32, status: &str, message: &str, data: Option<T>) -> HttpResponse
where
    T: Serialize,
{
    let response = json!({
        "code": code,
        "status": status,
        "message": message,
        "data": ResponseData { data }
    });

    HttpResponse::Ok().json(response)
}
