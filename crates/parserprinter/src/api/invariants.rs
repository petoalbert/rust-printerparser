use crate::db::db_ops::{Persistence, DB};

pub fn check_current_branch_current_commit_set(conn: &Persistence) {
    let current_branch = conn
        .read_current_branch_name()
        .expect("No current branch set");
    conn.read_branch_tip(&current_branch)
        .expect("Branch tip is not set for current branch");
    conn.read_current_latest_commit()
        .expect("No current branch set");
}
