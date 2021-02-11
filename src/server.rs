use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};
use std::path::PathBuf;

struct AppState {
    target: PathBuf,
}

async fn index(data: web::Data<AppState>) -> Result<NamedFile> {
    let target = &data.target;
    let path = target.join("index.html");

    Ok(NamedFile::open(path)?)
}

pub async fn run(target: PathBuf) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                target: target.clone(),
            })
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", &target).show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
