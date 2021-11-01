mod handlers;
mod models;

use std::{env, io};

use actix_web::web;
use actix_web::middleware;
use actix_web::{App, HttpServer};
use actix_web::web::{Data};

use actix_files::Files;

use mongodb::bson;
use mongodb::{Client, IndexModel};
use mongodb::options::IndexOptions;

const DB_NAME: &str = "axosh";
const COLUMN_NAME: &str = "shorturls";

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS")
        .expect("No server address was provided.");

    let client = {
        let mongodb_uri = env::var("MONGODB_URI")
            .expect("No mongodb uri was provided.");

        Client::with_uri_str(mongodb_uri)
            .await
            .expect("Unable to create client.")
    };

    {
        let options = IndexOptions::builder()
            .unique(true)
            .build();

        let model = IndexModel::builder()
            .keys(bson::doc!("id": 1))
            .options(options)
            .build();

        client
            .database(DB_NAME)
            .collection::<models::ShortUrl>(COLUMN_NAME)
            .create_index(model, None)
            .await
            .expect("Unable to create index.");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/url")
                            .service(handlers::get_url_by_id)
                            .service(handlers::add_url)
                    )
            )
            .service(handlers::get_url_by_id)
            .service(
                Files::new("/", "./static/root")
                    .index_file("index.html")
            )
    })
        .bind(server_address)?
        .run()
        .await
}