use actix_web::web;

use super::handlers::{find_by_id, get_profile, update_profile};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{user_id}", web::get().to(find_by_id))
            .route("/profile", web::put().to(update_profile))
            .route("/profile/{id}", web::get().to(get_profile)),
    );
}
