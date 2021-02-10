use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};
use std::path::Path;

async fn index() -> Result<NamedFile> {
    let path = Path::new("./outputs/index.html");

    Ok(NamedFile::open(path)?)
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "./outputs/").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
