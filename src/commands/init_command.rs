use crate::db_ops::{Persistence, DB};

pub fn run_init_command(db_path: &str) {
    let db = Persistence::open(db_path).expect("Cannot open DB");
    db.write_branch_tip("main", "initial")
        .expect("Cannot write default branch");
    db.write_current_branch_name("main")
        .expect("Cannot write current branch");
    db.write_current_latest_commit("initial")
        .expect("Cannot write current latest commit");
}
