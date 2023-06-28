use crate::db_ops::{Persistence, DB};

use super::invariants::check_current_branch_current_commit_set;

pub fn run_new_branch_command(db_path: &str, new_branch_name: String) {
    let db = Persistence::open(db_path).expect("Cannot open DB");

    check_current_branch_current_commit_set(&db);

    let current_brach_name = db
        .read_current_branch_name()
        .expect("Cannot read current branch name");

    if current_brach_name != "main" {
        println!("New branches can only be created if main is the current branch")
    }

    let tip = db
        .read_current_latest_commit()
        .expect("Cannot read current branch tip");

    db.write_branch_tip(&new_branch_name, &tip)
        .expect("Cannot create new branch");

    db.write_current_branch_name(&new_branch_name)
        .expect("Cannot set current branch name");

    db.write_current_latest_commit(&tip)
        .expect("Cannot set new branch name");
}
