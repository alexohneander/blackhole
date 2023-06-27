use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

mod handlers;
mod api;
mod helpers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(handlers::index::hello)
            .service(api::download::file)
            .service(api::upload::file)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}