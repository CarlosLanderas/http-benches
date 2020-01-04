use actix_web::{web, HttpServer, App, HttpResponse, Result};
use std::sync::{Mutex, Arc};
use failure::Error;
use serde::{Serialize, Deserialize};

static URL : &str =  "https://pokeapi.co/api/v2/ability";
static CONTENT_TYPE : &str = "application/json";

#[derive(Clone, Debug)]
struct Data {
    client: reqwest::Client,
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
            client: reqwest::Client::new(),
            counter: Arc::new(Mutex::new(0))
        }
    }

    fn get_counter(&self) -> Result<u32, Error> {

       let mux = self.counter.lock().unwrap();
       let mut counter = *mux;

       let curr = counter;
       counter += 1;

        Ok(curr)
    }
}

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .data(Data::new())
            .route("/call", web::post().to(post_call))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
async fn post_call(data: web::Data<&Data>)  -> Result<HttpResponse> {

    let id = data.get_counter().unwrap();
    let url = format!("{}/{}", URL, id);
    let ability : Ability = data.client.get(&url)
        .send()
        .await
        .map_err(Error::from)?
        .json()
        .await
        .map_err(Error::from)?;


    let json = serde_json::to_string(&ability)
        .map_err(Error::from)?;


    Ok(HttpResponse::Ok()
        .content_type(CONTENT_TYPE)
        .json(json))
}




