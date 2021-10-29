use std::{io, env};
use dotenv::dotenv;
use actix_web::{App, HttpServer};
use actix_web::middleware::NormalizePath;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS")
        .expect("No server address was provided.");

    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::default())
    })
        .bind(server_address)?
        .run()
        .await
}