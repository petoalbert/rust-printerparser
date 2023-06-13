use crate::db_ops::{open_db, read_commits};

pub fn run_log_command(db_path: &str) {
    let conn = open_db(db_path).expect("Cannot open the DB");
    let commits = read_commits(&conn).expect("Cannot read commits");
    for commit in commits {
        println!("{} {}", commit.hash, commit.message)
    }
}
