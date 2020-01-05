use actix_web::{web, HttpServer, App, HttpResponse, Result};
use std::sync::{Mutex, Arc};
use failure::Error;
use serde::{Serialize, Deserialize};
use surf::middleware::Body;

static URL : &str =  "https://pokeapi.co/api/v2/ability";
static CONTENT_TYPE : &str = "application/json";

#[derive(Clone, Debug)]
struct Data {
    counter: Arc<Mutex<u32>>,
}

#[derive(Clone, Debug,  Serialize, Deserialize)]
struct Ability {
    id: u32,
    name: String,
    is_main_series: bool,
}

impl Data {
    fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(1))
        }
    }

    fn get_counter(&self) -> Result<u32, Error> {

       let mut counter = self.counter.lock().unwrap();
       let curr = counter.clone();
        *counter +=1;

        Ok(curr)
    }
}

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let data = Data::new();

    HttpServer::new(move || {
        App::new()
            .route("/call", web::post().to(post_call))
            .data(data.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn post_call(data: web::Data<Data>)  -> Result<HttpResponse> {

    let id = data.get_counter().unwrap();
    let url = format!("{}/{}", URL, id);

    let ability = surf::get(url)
        .recv_json::<Ability>().
         await
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(CONTENT_TYPE)
        .json(ability))
}




