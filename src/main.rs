use std::{io, env};
use dotenv::dotenv;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS")
        .expect("No server address was provided.");

    HttpServer::new(|| {
        App::new()
    })
        .bind(server_address)?
        .run()
        .await
}
