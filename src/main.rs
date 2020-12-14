mod models;
mod web_services;
mod todo_logic;
mod repository;
mod database_engine_pool;
mod schema;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use crate::database_engine_pool::{connect, MysqlPool};

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(connect())
            .service(web_services::create_task)
            .service(web_services::update_task)
            .service(web_services::delete_task)
            .service(web_services::get_all)
            //.service(web_services::get)
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .await
}
