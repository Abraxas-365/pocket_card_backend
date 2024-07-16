use actix_web::web;

use super::handlers::{
    find_by_id, find_settings_by_profile_id, get_profile, update_profile, update_settings,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{user_id}", web::get().to(find_by_id))
            .route("/profile", web::put().to(update_profile))
            .route("/profile/{id}", web::get().to(get_profile))
            .route(
                "/settings/{user_id}",
                web::get().to(find_settings_by_profile_id),
            )
            .route("/settings", web::put().to(update_settings)),
    );
}
