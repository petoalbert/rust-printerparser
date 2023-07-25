#[cfg(test)]
pub fn init_db(db_path: &str) {
    use super::init_command::init_db;

    init_db(db_path).expect("Cannot init DB")
}

#[cfg(test)]
pub fn commit(db_path: &str, message: &str, blend_path: &str) {
    use super::commit_command::create_new_commit;

    create_new_commit(blend_path, db_path, Some(message.to_owned()))
        .expect("Cannot create new commit")
}

#[cfg(test)]
pub fn new_branch(db_path: &str, name: &str) {
    use super::new_branch_command::create_new_branch;

    create_new_branch(db_path, name).expect("Cannot create new branch")
}
