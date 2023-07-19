use actix_web::{App, HttpServer};

use super::endpoints::{checkpoints, commit, hello, restore};

pub async fn serve() {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(commit)
            .service(checkpoints)
            .service(restore)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Cannot bind to 127.0.0.1:8080")
    .run()
    .await
    .unwrap()
}
