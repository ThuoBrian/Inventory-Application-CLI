use actix_web::{HttpResponse, Responder, post, web};

#[derive(serde::Deserialize, Debug)]
struct AuthData {
    fullname: String,
    username: String,
    password: String,
    email: String,
}

#[post("/auth/signup")]
pub async fn signup(data: web::Json<AuthData>) -> impl Responder {
    HttpResponse::Ok().body(format!("SIGNED UP: {:?}", data))
}

#[post("/auth/signin")]
pub async fn signin(data: web::Json<AuthData>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "User '{}': Fullname '
{}': signed in",
        data.username, data.fullname
    ))
}
