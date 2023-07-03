use crate::db_ops::{Persistence, ShortCommitRecord, DB};

pub fn log_checkpoints(db_path: &str, branch_name: Option<String>) -> Vec<ShortCommitRecord> {
    let conn = Persistence::open(db_path).expect("Cannot open the DB");

    if let Some(branch) = branch_name {
        conn.read_commits_for_branch(&branch)
            .expect("Cannot read commits")
    } else {
        conn.read_all_commits().expect("Cannot read commits")
    }
}
