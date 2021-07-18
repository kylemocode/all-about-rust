mod config;
mod models;
mod handlers;
mod db;

// use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()>{

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap(); // unwrap -> error handling

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("start server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // Make a copy of reference
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
