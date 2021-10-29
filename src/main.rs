mod handlers;

use std::{io, env};
use dotenv::dotenv;

use actix_web::{App, HttpServer};
use actix_web::web::scope;
use actix_web::middleware::NormalizePath;

use crate::handlers::{add_url, get_url_by_id};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS")
        .expect("No server address was provided.");

    HttpServer::new(|| {
        App::new()
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