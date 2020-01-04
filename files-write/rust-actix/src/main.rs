use actix_web::{HttpServer, App, Result, post, HttpResponse};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let files_folder = Path::new("files");

    if !files_folder.exists() {
        println!("Creating files folder");
        fs::create_dir(files_folder).unwrap();
    }

    HttpServer::new(|| {
        App::new()
            .service(post_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/file")]
async fn post_file() -> Result<HttpResponse> {

    let id = Uuid::new_v4().to_string();
    let contents = format!("The content of the file is {}", id);
    let file_path = format!("files/{}", id);
    let path = Path::new(&file_path);

    fs::write(path, contents).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(id))
}