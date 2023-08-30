use super::health_controller::health_checker_handler;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_checker_handler);
}