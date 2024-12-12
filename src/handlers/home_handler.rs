use actix_web::{HttpResponse, Responder};

pub async fn get_home() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "code": 200,
        "message": "Welcome to the Home API!",
        "data": {
            "title": "Home Page",
            "description": "This is the home page API endpoint."
        }
    }))
}
