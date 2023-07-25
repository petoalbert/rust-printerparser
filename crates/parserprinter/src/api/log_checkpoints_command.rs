use crate::db::db_ops::{DBError, Persistence, ShortCommitRecord, DB};

pub fn log_checkpoints(
    db_path: &str,
    branch_name: Option<String>,
) -> Result<Vec<ShortCommitRecord>, DBError> {
    let conn = Persistence::open(db_path)?;

    if let Some(branch) = branch_name {
        conn.read_commits_for_branch(&branch)
    } else {
        conn.read_all_commits()
    }
}
