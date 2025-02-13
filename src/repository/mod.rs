use actix_web::web;

pub mod action;
pub mod admin;
mod api;
mod badge;
pub mod controller;
pub mod maven;
pub mod models;
pub mod repository;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::browse)
        .service(controller::browse_storage)
        .service(controller::get_repository)
        .service(controller::post_repository)
        .service(controller::patch_repository)
        .service(controller::put_repository)
        .service(controller::head_repository)
        .service(api::get_versions)
        .service(badge::badge);
}
