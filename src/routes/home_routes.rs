use actix_web::web;

use crate::handlers::home_handler::get_home;

pub fn home_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/home") // Prefix untuk semua route di halaman home
            .route("", web::get().to(get_home)) // Endpoint GET untuk "/home/"
    );
}
