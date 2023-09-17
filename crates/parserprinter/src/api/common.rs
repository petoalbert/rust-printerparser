use crate::db::db_ops::{DBError, Persistence, DB};

pub fn read_latest_commit_hash_on_branch(
    conn: &Persistence,
    branch_name: &str,
) -> Result<String, DBError> {
    conn.read_branch_tip(branch_name)
        .and_then(|tip| tip.ok_or(DBError::Error("Branch tip does not exist".to_owned())))
}
