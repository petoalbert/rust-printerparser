use actix_web::{App, HttpServer};

use super::endpoints::{commit, hello};

pub async fn serve() {
    HttpServer::new(|| App::new().service(hello).service(commit))
        .bind(("127.0.0.1", 8080))
        .expect("Cannot bind to 127.0.0.1:8080")
        .run()
        .await
        .unwrap()
}
