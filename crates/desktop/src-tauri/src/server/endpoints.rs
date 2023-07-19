use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use parserprinter::api::{
    commit_command::create_new_commit,
    log_checkpoints_command::log_checkpoints,
    restore_command::{self, restore_checkpoint},
};
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
pub struct CommitPayload {
    db_path: String,
    file_path: String,
    message: String,
}

#[post("/commit")]
pub async fn commit(data: Json<CommitPayload>) -> impl Responder {
    let result = create_new_commit(
        &data.file_path,
        &data.db_path,
        Some(data.message.to_owned()),
    );

    match result {
        Err(_) => HttpResponse::BadRequest(),
        Ok(_) => HttpResponse::Ok(),
    }
}

#[derive(Serialize)]
struct ShortCommitPayload {
    hash: String,
    message: String,
}

#[get("/checkpoints/{db_path}")]
pub async fn checkpoints(path: web::Path<(String,)>) -> impl Responder {
    let checkpoints: Vec<ShortCommitPayload> = log_checkpoints(&path.0.to_owned(), None)
        .into_iter()
        .map(|checkpoint| ShortCommitPayload {
            hash: checkpoint.hash,
            message: checkpoint.message,
        })
        .collect();

    HttpResponse::Ok().json(checkpoints)
}

#[derive(Deserialize)]
pub struct RestorePayload {
    db_path: String,
    file_path: String,
    hash: String,
}

#[post("/restore")]
pub async fn restore(data: Json<RestorePayload>) -> impl Responder {
    let result = restore_checkpoint(&data.file_path, &data.db_path, &data.hash);

    match result {
        Err(_) => HttpResponse::BadRequest(),
        Ok(_) => HttpResponse::Ok(),
    }
}
