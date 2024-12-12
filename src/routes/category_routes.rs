use actix_web::web;
use crate::handlers::category_handler::get_categories;

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/categories")
            .route(web::get().to(get_categories))
    );
}
