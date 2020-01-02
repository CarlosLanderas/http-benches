use actix_web::{HttpServer, App, get, web, Responder, HttpRequest, Error, HttpResponse};
use serde::{Serialize};
use actix_web::body::Body;
use futures::future::{ready, Ready};
use uuid::Uuid;
use rand::Rng;
use rand::distributions::Alphanumeric;


#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Serialize, Debug)]
struct User {
    id: String,
    email: String,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let json = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json)))
    }
}

#[get("/user")]
async fn get_user() -> impl Responder {

    let id = Uuid::new_v4().to_string();
    let email : String = rand::thread_rng().sample_iter(&Alphanumeric).take(20).collect();
    return User {
        email: format!("{}@host.com", email),
        id,
    }
}