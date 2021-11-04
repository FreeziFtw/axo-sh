use actix_web::web;
use actix_web::{Error, HttpResponse};
use actix_web::http::header;

use mongodb::Client;
use mongodb::bson;

use rand::distributions;
use rand::Rng;

use crate::{models, actions, COLUMN_NAME, DB_NAME};

#[actix_web::post("/")]
pub async fn add_url(client: web::Data<Client>, url: web::Json<models::Url>) -> Result<HttpResponse, Error> {
    let url = &url.url;

    if url.len() > 2048 {
        return Ok(
            HttpResponse::BadRequest()
                .json(models::ErrorMessage::new("URL exceeds length limit"))
        );
    }

    let url = url::Url::parse(url)
        .map_err(|_| {
            HttpResponse::BadRequest()
                .json(models::ErrorMessage::new("Invalid URL format"))
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
    let url = actions::get_url_by_id(client, id).await?;

    Ok(
        match url {
            None => HttpResponse::NotFound().finish(),
            Some(url) => HttpResponse::Ok().json(models::Url { url: url.url }),
        }
    )
}

#[actix_web::get("/{id}/")]
pub async fn get_url_redirect_by_id(client: web::Data<Client>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let url = actions::get_url_by_id(client, id).await?;

    Ok(
        match url {
            None => HttpResponse::NotFound().finish(),
            Some(url) => HttpResponse::PermanentRedirect().header(header::LOCATION, url.url).finish().into_body(),
        }
    )
}
