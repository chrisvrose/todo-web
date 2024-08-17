use std::sync::Arc;

use actix_web::{web::Data, App, HttpServer};
use simple_logger::SimpleLogger;
use sqlx::{PgPool, Postgres};
use web::todo::repository::PostgresTodoRepository;

mod util;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv()
        .inspect_err(|e| {
            println!("No .env - {}", e);
        })
        .ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .expect("Failed to init logger");

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed connecting to db url");

    let pool_arc = Arc::new(pool);

    HttpServer::new(move || {
        let todo_repo = PostgresTodoRepository::new(pool_arc.clone());
        App::new()
            .service(web::todo::api::get_list)
            .app_data(Data::new(todo_repo))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
