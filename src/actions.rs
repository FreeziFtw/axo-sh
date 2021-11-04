use actix_web::web;
use actix_web::{Error, HttpResponse};

use mongodb::Client;
use mongodb::bson;

use crate::{models, COLUMN_NAME, DB_NAME};
use crate::models::ShortUrl;

pub async fn get_url_by_id(
    client: web::Data<Client>,
    id: web::Path<String>
) -> Result<Option<ShortUrl>, Error> {
    let collection = client
        .database(DB_NAME)
        .collection::<models::ShortUrl>(COLUMN_NAME);

    Ok(
        collection.find_one(bson::doc!("id": id.0), None).await
            .map_err(|_| {
                HttpResponse::InternalServerError().finish()
            })?
    )
}