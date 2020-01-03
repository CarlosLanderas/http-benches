#[macro_use]
extern crate bson;

use actix_web::{post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: ObjectId,
    pub name: String,
}

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let uri = "mongodb://root:example@localhost:27017";
    let mongo_db = Client::with_uri_str(uri).unwrap();
    let db = mongo_db.database("main");

    db.create_collection("books", None).unwrap();

    let books = db.collection("books");
    
    HttpServer::new(move || {
        App::new()
            .data(books.clone())
            .service(post_book)
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/books")]
async fn post_book(books: web::Data<Collection>) -> Result<HttpResponse> {
    let id = Uuid::new_v4().to_string();
    let book_name = format!("Book{}", id);

    books.insert_one(
            doc! {
                "name": book_name
            },
            None,
        )
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body::<String>(id.into()))
}
