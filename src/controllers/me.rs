use actix_web::{Responder, get};

#[get("/me")]
pub async fn get_profile() -> impl Responder {
    "me"
}

#[get("/me")]
pub async fn update_profile() -> impl Responder {
    "update me"
}
