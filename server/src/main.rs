use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

mod handlers;
mod api;
mod helpers;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Initialize database
    let db = database::context::get_db().await;
    database::context::init_database(db).await;

    HttpServer::new(|| {
        App::new()
            // Register Middlewares
            .wrap(Logger::default())
            // Register Routes
            .service(handlers::index::hello)
            .service(api::download::file)
            .service(api::upload::file)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}