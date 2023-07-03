#[cfg(test)]
pub fn init_db(db_path: &str) {
    use super::init_command::run_init_command;

    run_init_command(db_path)
}

#[cfg(test)]
pub fn commit(db_path: &str, message: &str, blend_path: &str) {
    use super::commit_command::run_commit_command;

    run_commit_command(blend_path, db_path, Some(message.to_owned()))
}

#[cfg(test)]
pub fn new_branch(db_path: &str, name: &str) {
    use super::new_branch_command::run_new_branch_command;

    run_new_branch_command(db_path, name.to_string())
}
