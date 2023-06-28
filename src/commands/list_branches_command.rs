use crate::db_ops::{Persistence, DB};

pub fn run_list_branches(db_path: &str) {
    let db = Persistence::open(db_path).expect("Cannot open db");
    let branches = db
        .read_all_branches()
        .expect("Cannot read branches from DB");
    
    for branch in branches {
        println!("{:?}", branch)
    }
}
