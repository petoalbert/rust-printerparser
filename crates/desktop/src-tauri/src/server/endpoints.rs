use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use parserprinter::api::{
    commit_command::create_new_commit, get_current_branch, list_branches_command::list_braches,
    log_checkpoints_command::log_checkpoints, new_branch_command::create_new_branch,
    restore_command::restore_checkpoint, switch_command::switch_branches,
};
use serde::{Deserialize, Serialize};

use crate::serde_instances::DBErrorWrapper;

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json("Running")
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

#[get("/checkpoints/{db_path}/{branch}")]
pub async fn checkpoints(path: web::Path<(String, String)>) -> impl Responder {
    let result = log_checkpoints(&path.0.to_owned(), Some(path.1.to_owned()));

    match result {
        Ok(checkpoints) => HttpResponse::Ok().json(
            checkpoints
                .into_iter()
                .map(|checkpoint| ShortCommitPayload {
                    hash: checkpoint.hash,
                    message: checkpoint.message,
                })
                .collect::<Vec<ShortCommitPayload>>(),
        ),
        Err(err) => HttpResponse::BadRequest().json(DBErrorWrapper(err)),
    }
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

#[get("/branches/{db_path}")]
pub async fn branches(path: web::Path<(String,)>) -> impl Responder {
    let result = list_braches(&path.0.to_owned());
    match result {
        Ok(branches) => HttpResponse::Ok().json(branches),
        Err(err) => HttpResponse::BadRequest().json(DBErrorWrapper(err)),
    }
}

#[derive(Deserialize)]
pub struct NewBranchPayload {
    db_path: String,
    branch_name: String,
}

#[post("/branches/new")]
pub async fn new_branch(data: Json<NewBranchPayload>) -> impl Responder {
    let result = create_new_branch(&data.db_path, &data.branch_name);
    match result {
        Err(_) => HttpResponse::BadRequest(),
        Ok(_) => HttpResponse::Ok(),
    }
}

#[derive(Deserialize)]
pub struct SwitchBranchPayload {
    db_path: String,
    branch_name: String,
    file_path: String,
}

#[post("/branches/switch")]
pub async fn switch_branch(data: Json<SwitchBranchPayload>) -> impl Responder {
    let result = switch_branches(&data.db_path, &data.branch_name, &data.file_path);
    match result {
        Err(_) => HttpResponse::BadRequest(),
        Ok(_) => HttpResponse::Ok(),
    }
}

#[get("/branches/current/{db_path}")]
pub async fn read_current_branch(path: web::Path<(String,)>) -> impl Responder {
    let result = get_current_branch::get_current_branch(&path.0.to_owned());
    match result {
        Err(err) => HttpResponse::BadRequest().json(DBErrorWrapper(err)),
        Ok(branch) => HttpResponse::Ok().json(branch),
    }
}
