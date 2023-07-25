use crate::db::db_ops::{DBError, Persistence, DB};

pub fn check_current_branch_current_commit_set(conn: &Persistence) -> Result<(), DBError> {
    let current_branch = conn.read_current_branch_name()?;
    conn.read_branch_tip(&current_branch)?;
    conn.read_current_latest_commit()?;
    Ok(())
}
