use actix_web::{get, Responder, HttpResponse, http::{StatusCode}};

#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    "I'm fine. Thanks for asking !"
}

pub async fn forbidden() -> impl Responder {
    HttpResponse::build(StatusCode::from_u16(503).unwrap()).body("Hey there, fuck off !")
}
