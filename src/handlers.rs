use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web::http::header;

use mongodb::Client;
use mongodb::bson;

use rand::distributions;
use rand::Rng;

use crate::{models, COLUMN_NAME, DB_NAME};

#[actix_web::post("/")]
pub async fn add_url(client: web::Data<Client>, url: web::Json<models::Url>) -> Result<HttpResponse, Error> {
    let url = url::Url::parse(&url.url)
        .map_err(|_| {
            HttpResponse::BadRequest().finish()
        })?.to_string();

    let collection = client
        .database(DB_NAME)
        .collection::<models::ShortUrl>(COLUMN_NAME);

    let id = loop {
        let id: String = rand::thread_rng()
            .sample_iter(&distributions::Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let url = collection.find_one(bson::doc!("id": &id), None)
            .await
            .map_err(|_| {
                HttpResponse::InternalServerError().finish()
            })?;

        match url {
            None => break id,
            Some(_) => continue,
        }
    };

    collection.insert_one(models::ShortUrl { id: id.clone(), url }, None).await
        .map_err(|_| {
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(models::Id { id }))
}

#[actix_web::get("/{id}/")]
pub async fn get_url_by_id(client: web::Data<Client>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let collection = client
        .database(DB_NAME)
        .collection::<models::ShortUrl>(COLUMN_NAME);

    let url = collection.find_one(bson::doc!("id": id.0), None).await
        .map_err(|_| {
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(
        match url {
            None => HttpResponse::NotFound().finish(),
            Some(url) => HttpResponse::PermanentRedirect().header(header::LOCATION, url.url).finish().into_body(),
        }
    )
}
