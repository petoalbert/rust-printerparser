use crate::db::db_ops::{Persistence, DB};

pub fn list_braches(db_path: &str) -> Vec<String> {
    let db = Persistence::open(db_path).expect("Cannot open db");
    db.read_all_branches()
        .expect("Cannot read branches from DB")
}
