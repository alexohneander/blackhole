use actix_web::{get, Responder, HttpResponse, web};

use crate::helpers;

#[get("/api/download/{id}")]
pub async fn file(id: web::Path<String>) -> impl Responder {
    print!("{}", id);

    let token = helpers::token::generate_troken();
    HttpResponse::Ok().body(token)
}