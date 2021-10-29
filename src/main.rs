mod handlers;
mod models;

#[macro_use]
extern crate diesel;

use std::{io, env};
use dotenv::dotenv;

use actix_web::{App, HttpServer};
use actix_web::web::scope;
use actix_web::middleware::NormalizePath;

use diesel::MysqlConnection;
use diesel::r2d2::{Pool, ConnectionManager};

use crate::handlers::{add_url, get_url_by_id};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS")
        .expect("No server address was provided.");

    let database_url = env::var("DATABASE_URL")
        .expect("No database url was provided.");

    let manager =
        ConnectionManager::<MysqlConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Unable to build pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(NormalizePath::default())
            .service(
                scope("/api")
                    .service(
                        scope("/url")
                            .service(add_url)
                            .service(get_url_by_id)
                    )
            )
            .service(get_url_by_id)
    })
        .bind(server_address)?
        .run()
        .await
}