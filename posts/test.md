# Jakiś tytuł

Jakiś tekst

```rs
#[macro_use]
extern crate lazy_static;

mod build;
mod format;
mod highlight;
mod posts;
mod server;
mod templates;
mod watch;

#[actix_web::main]
async fn main() {
    build::build().expect("Failed to build");

    actix::spawn(async {
        watch::watch(build::build).expect("failed to watch");
    });

    server::run().await.expect("failed to start server");
}
```