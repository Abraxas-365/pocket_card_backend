use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::{error::ApiError, modules::user::Service};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    google_id: String,
    email: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    email: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    email: Option<String>,
    name: Option<String>,
    bio: Option<String>,
    phone_number: Option<String>,
    location: Option<String>,
    profile_picture_url: Option<String>,
    theme: Option<String>,
    custom_url: Option<String>,
    job_title: Option<String>,
    facebook_url: Option<String>,
    twitter_url: Option<String>,
    instagram_url: Option<String>,
    linkedin_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSettingsRequest {
    exchange_contacts: Option<bool>,
    save_contact: Option<bool>,
    call_me: Option<bool>,
    email_me: Option<bool>,
    social_media: Option<bool>,
}

pub async fn find_by_id(
    service: web::Data<Arc<Service>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let user = service.find_by_id(*user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_profile(
    service: web::Data<Arc<Service>>,
    session: Session,
    profile_data: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = session.get::<i32>("user_id").unwrap().unwrap(); // TODO: Handle session error properly

    let mut profile = service.find_profile_by_user_id(user_id).await?;

    if let Some(email) = &profile_data.email {
        profile.email = Some(email.clone());
    }
    if let Some(name) = &profile_data.name {
        profile.name = Some(name.clone());
    }
    if let Some(bio) = &profile_data.bio {
        profile.bio = Some(bio.clone());
    }
    if let Some(phone_number) = &profile_data.phone_number {
        profile.phone_number = Some(phone_number.clone());
    }
    if let Some(location) = &profile_data.location {
        profile.location = Some(location.clone());
    }
    if let Some(profile_picture_url) = &profile_data.profile_picture_url {
        profile.profile_picture_url = Some(profile_picture_url.clone());
    }
    if let Some(theme) = &profile_data.theme {
        profile.theme = Some(theme.clone());
    }
    if let Some(custom_url) = &profile_data.custom_url {
        profile.custom_url = Some(custom_url.clone());
    }
    if let Some(job_title) = &profile_data.job_title {
        profile.job_title = Some(job_title.clone());
    }
    if let Some(facebook_url) = &profile_data.facebook_url {
        profile.facebook_url = Some(facebook_url.clone());
    }
    if let Some(twitter_url) = &profile_data.twitter_url {
        profile.twitter_url = Some(twitter_url.clone());
    }
    if let Some(instagram_url) = &profile_data.instagram_url {
        profile.instagram_url = Some(instagram_url.clone());
    }
    if let Some(linkedin_url) = &profile_data.linkedin_url {
        profile.linkedin_url = Some(linkedin_url.clone());
    }

    let updated_profile = service.update_profile(&profile).await?;
    Ok(HttpResponse::Ok().json(updated_profile))
}

pub async fn get_profile(
    service: web::Data<Arc<Service>>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let profile = service.find_profile_by_id(&id).await?;
    Ok(HttpResponse::Ok().json(profile))
}

pub async fn find_settings_by_profile_id(
    service: web::Data<Arc<Service>>,
    profile_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let settings = service.find_settings_by_profile_id(&profile_id).await?;
    Ok(HttpResponse::Ok().json(settings))
}

pub async fn update_settings(
    service: web::Data<Arc<Service>>,
    session: Session,
    settings_data: web::Json<UpdateSettingsRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = session.get::<i32>("user_id").unwrap().unwrap(); // TODO: Handle session error properly

    let mut settings = service.find_settings_by_user_id(user_id).await?;

    if let Some(exchange_contacts) = settings_data.exchange_contacts {
        settings.exchange_contacts = exchange_contacts;
    }
    if let Some(save_contact) = settings_data.save_contact {
        settings.save_contact = save_contact;
    }
    if let Some(call_me) = settings_data.call_me {
        settings.call_me = call_me;
    }
    if let Some(email_me) = settings_data.email_me {
        settings.email_me = email_me;
    }
    if let Some(social_media) = settings_data.social_media {
        settings.social_media = social_media;
    }

    let updated_settings = service.update_settings(&settings).await?;
    Ok(HttpResponse::Ok().json(updated_settings))
}
