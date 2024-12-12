use actix_web::{web};
use crate::handlers::publishers_handler::get_publishers;
use crate::handlers::publishers_handler::get_publishers_by_category;
use crate::handlers::publishers_handler::get_publishers_by_author;
use crate::handlers::publishers_handler::get_publishers_featured;
use crate::handlers::publishers_handler::search_publishers;

pub fn publishers_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/publishers")
            .route("", web::get().to(get_publishers))
            .route("/featured", web::get().to(get_publishers_featured))
            .route("/category/{category_id}", web::get().to(get_publishers_by_category))
            .route("/author/{author_id}", web::get().to(get_publishers_by_author))
            .route("/search", web::get().to(search_publishers))
    );
}
