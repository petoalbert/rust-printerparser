use crate::db_ops::{Persistence, DB};

pub fn check_current_branch_current_commit_set(conn: &Persistence) {
    let current_branch = conn
        .read_current_branch_name()
        .expect("No current branch set");
    conn.read_branch_tip(&current_branch)
        .expect("Branch tip is not set for current branch");
    conn.read_current_latest_commit()
        .expect("No current branch set");
}

pub fn check_no_detached_head_invariant(conn: &Persistence) {
    let current_branch = conn
        .read_current_branch_name()
        .expect("No current branch set");
    let latest_commit_for_branch = conn
        .read_branch_tip(&current_branch)
        .expect("Branch tip is not set for current branch");
    let current_commit = conn
        .read_current_latest_commit()
        .expect("No current branch set");

    if current_commit != latest_commit_for_branch {
        panic!("The current commit is not the same as the latest commit for the branch")
    }
}
