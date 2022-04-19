use actix_web::{App, HttpServer, web};
use crate::models::Status;
use crate::config::Config;
use crate::handler::*;

mod models;
mod config;
mod handler;
mod db;

use dotenv::dotenv;
use tokio_postgres::NoTls;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();
    //let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting Server at {}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}/{_:/?}", web::put().to(check_item))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}