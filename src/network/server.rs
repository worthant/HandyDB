use actix_web::web;

use crate::network::services::{set_service, get_service};

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/set").route(web::post().to(set_service::set)),
    );
    cfg.service(
        web::resource("/get/{key}").route(web::get().to(get_service::get)),
    );
}
