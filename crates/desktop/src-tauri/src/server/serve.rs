use actix_web::{App, HttpServer};

use super::endpoints::{
    branches, checkpoints, commit, hello, new_branch, read_current_branch, restore, switch_branch,
};

pub async fn serve() {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(commit)
            .service(checkpoints)
            .service(restore)
            .service(branches)
            .service(new_branch)
            .service(switch_branch)
            .service(read_current_branch)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Cannot bind to 127.0.0.1:8080")
    .run()
    .await
    .unwrap()
}
