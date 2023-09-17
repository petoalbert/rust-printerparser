#[cfg(test)]
pub fn init_db(db_path: &str, project_id: &str) {
    use super::init_command::init_db;

    init_db(db_path, project_id).expect("Cannot init DB")
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

#[cfg(test)]
use crate::db::db_ops::ShortCommitRecord;
use crate::db::structs::{BlockRecord, Commit};

#[cfg(test)]
pub fn list_checkpoints(db_path: &str, branch: &str) -> Vec<ShortCommitRecord> {
    use super::log_checkpoints_command;

    log_checkpoints_command::list_checkpoints(db_path, branch).expect("Cannot list checkpoints")
}

#[cfg(test)]
struct SimpleCommit {
    hash: String,
    prev_hash: String,
    branch: String,
    message: String,
    blocks: String,
}

#[cfg(test)]
struct SimpleTimeline {
    project_id: String,
    author: String,
    blocks: Vec<String>,
}
#[cfg(test)]
struct SimpleTimelineResult {
    commits: Vec<Commit>,
    blocks: Vec<BlockRecord>,
}

// #[cfg(test)]
// pub fn from_simple_timeline(simple_timeline: SimpleTimeline) {

// }
