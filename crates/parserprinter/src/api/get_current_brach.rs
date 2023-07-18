use crate::db::db_ops::{Persistence, DB};

pub fn get_current_brach(db_path: &str) -> String {
    let conn = Persistence::open(db_path).expect("Cannot open DB");
    conn.read_current_branch_name()
        .expect("Cannot read current branch name")
}
