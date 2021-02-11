mod args;
mod assets;
mod build;
mod format;
mod highlight;
mod index;
mod posts;
mod server;
mod templates;
mod watch;

#[actix_web::main]
async fn main() {
    args::Args::run().await.expect("Failed to run");
}
