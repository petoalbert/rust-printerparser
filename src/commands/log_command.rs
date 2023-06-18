use crate::db_ops::{SqliteDB, DB};

pub fn run_log_command(db_path: &str) {
    let conn = SqliteDB::open(db_path).expect("Cannot open the DB");
    let commits = conn.read_all_commits().expect("Cannot read commits");
    for commit in commits {
        println!("{} {}", commit.hash, commit.message)
    }
}
