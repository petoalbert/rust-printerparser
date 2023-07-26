use crate::db::db_ops::{DBError, Persistence, ShortCommitRecord, DB};

pub fn list_checkpoints(
    db_path: &str,
    branch_name: &str,
) -> Result<Vec<ShortCommitRecord>, DBError> {
    let conn = Persistence::open(db_path)?;
    let tip = conn
        .read_branch_tip(branch_name)?
        .ok_or(DBError::Consistency(format!(
            "Cannot read tip for branch {}",
            branch_name
        )))?;
    conn.read_ancestors_of_commit(&tip)
}
