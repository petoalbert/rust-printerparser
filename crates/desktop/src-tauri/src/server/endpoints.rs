use actix_web::{get, post, web::Json, HttpResponse, Responder};
use parserprinter::api::commit_command::create_new_commit;
use serde::Deserialize;

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
