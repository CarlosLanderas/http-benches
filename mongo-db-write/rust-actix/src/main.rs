#[macro_use]
extern crate bson;

use actix_web::{post, get, web, App, HttpResponse, HttpServer, Result};
use bson::doc;
use mongodb::{Client, Collection};
use uuid::Uuid;

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let uri = "mongodb://root:example@localhost:27017";
    let mongo_db = Client::with_uri_str(uri).unwrap();
    let db = mongo_db.database("main");

    match db.list_collection_names(None)
        .unwrap()
        .contains(&"books".to_owned()) {
        false =>  {
            println!("Creating books collection");
            db.create_collection("books", None).unwrap()
        },
        _ => ()
    }

    let books = db.collection("books");
    
    HttpServer::new(move || {
        App::new()
            .data(books.clone())
            .service(post_book)
            .service(get_books)
            
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

#[get("/books")]
async fn get_books(books: web::Data<Collection>) -> Result<HttpResponse> {

    let cursor = books.find(doc!{}, None).unwrap();
    let books : Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    let json = serde_json::to_string(&books).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(json))
}
