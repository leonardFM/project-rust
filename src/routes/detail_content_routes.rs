// src/routes/detail_content_routes.rs
use actix_web::web;
use crate::handlers::detail_content_handler::get_detail_content_by_id;

pub fn detail_content_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/detail-content/{id}")
            .route(web::get().to(get_detail_content_by_id))  // Mengambil content berdasarkan id
    );
}
