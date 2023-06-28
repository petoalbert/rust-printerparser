use crate::db_ops::{Persistence, DB};

pub fn run_log_checkpoints_command(db_path: &str, branch_name: Option<String>) {
    let conn = Persistence::open(db_path).expect("Cannot open the DB");

    let commits = if let Some(branch) = branch_name {
        conn.read_commits_for_branch(&branch)
            .expect("Cannot read commits")
    } else {
        conn.read_all_commits().expect("Cannot read commits")
    };

    for commit in commits {
        println!("{} {} {}", commit.hash, commit.branch, commit.message)
    }
}
