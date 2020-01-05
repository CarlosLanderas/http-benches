use hyper::{Server, Request, Body, Client, Response, Method, StatusCode};
use hyper::service::{service_fn, make_service_fn};
use hyper_tls::HttpsConnector;
use std::sync::{Mutex, Arc};
use serde_derive::{Serialize, Deserialize};
use hyper::client::{ResponseFuture};


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static URL : &str =  "https://pokeapi.co/api/v2/ability";
static CONTENT_TYPE : &str = "application/json";
static NOT_FOUND: &[u8] = b"Not Found";


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Ability {
    id: u32,
    name: String,
    is_main_series: bool,
}


#[derive(Clone)]
struct Data {
    counter: Arc<Mutex<u32>>,
}


impl Data {
    fn get_counter(&self) -> u32 {
        let mut counter = self.counter.lock().unwrap();
        let current = counter.clone();
        *counter += 1;
        current
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    env_logger::init();

    let addr = "127.0.0.1:8080".parse().unwrap();

    let data = Data{
        counter: Arc::new(Mutex::new(1)),
    };

    let service = make_service_fn(move |_| {

        let data = data.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                handle_request(req, data.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    server.await?;

    Ok(())
}

async fn handle_request(req: Request<Body>, data : Data) -> Result<Response<Body>> {

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/call")  => {

            let res: ResponseFuture;
            let counter = data.get_counter();

            {
                let https = HttpsConnector::new();
                let client = Client::builder().build::<_, hyper::Body>(https);
                res = client.get(format!("{}/{}", URL, counter).parse().unwrap());
            }

            let body = res.await.unwrap();

            let bytes = hyper::body::to_bytes(body.into_body()).await.unwrap();
            let ability: Ability = serde_json::from_slice(bytes.as_ref()).unwrap();

            let json = serde_json::to_string(&ability).unwrap();

            let response = Response::builder()
                .header("Content-Type", CONTENT_TYPE)
                .body(Body::from(json)).unwrap();

            Ok(response)
        },
        _ =>  Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOT_FOUND.into())
                .unwrap())
    }

}