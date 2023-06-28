use super::{
    commit_command::run_commit_command, init_command::run_init_command,
    new_branch_command::run_new_branch_command,
};

pub fn init_db(db_path: &str) {
    run_init_command(db_path)
}

pub fn commit(db_path: &str, message: &str, blend_path: &str) {
    run_commit_command(blend_path, db_path, Some(message.to_owned()))
}

pub fn new_branch(db_path: &str, name: &str) {
    run_new_branch_command(db_path, name.to_string())
}
