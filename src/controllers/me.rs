use actix_web::{HttpResponse, Responder, delete, get, put, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

/// Get current user profile
#[get("/me")]
pub async fn get_profile() -> impl Responder {
    // TODO: Fetch from database
    let profile = UserProfile {
        id: "123".to_string(),
        name: "Brian".to_string(),
        email: "thuogachau@gmail.com".to_string(),
    };

    HttpResponse::Ok().json(profile)
}

/// Update current user profile
#[put("/me")]
pub async fn update_profile(body: web::Json<UpdateProfileRequest>) -> impl Responder {
    // TODO: Validate and update in database
    let updated = UserProfile {
        id: "123".to_string(),
        name: body.name.clone().unwrap_or_else(|| "Brian".to_string()),
        email: body
            .email
            .clone()
            .unwrap_or_else(|| "thuogachau@gmail.com".to_string()),
    };

    HttpResponse::Ok().json(updated)
}

/// Delete current user profile
#[delete("/me")]
pub async fn delete_profile() -> impl Responder {
    // TODO: Soft delete or remove from database
    HttpResponse::NoContent().finish()
}
