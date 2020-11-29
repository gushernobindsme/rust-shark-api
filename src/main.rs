#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

mod db;
mod error_handler;
mod model;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    db::init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run();
    println!("Server running at http://127.0.0.1:8080/");

    server.await
}
