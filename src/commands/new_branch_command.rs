use crate::db_ops::{Persistence, DB};

pub fn run_new_branch_command(db_path: &str, branch_name: String) {
    let db = Persistence::open(db_path).expect("Cannot open DB");
    let current_brach_name = db
        .read_current_branch_name()
        .expect("Cannot read current branch name");
    if current_brach_name != "main" {
        println!("New branches can only be created if main is the current branch")
    }

    let tip = db
        .read_branch_tip(current_brach_name)
        .expect("Cannot read current branch tip");

    db.write_branch_tip(&branch_name, &tip)
        .expect("Cannot create new branch");
    db.write_current_branch_name(branch_name)
        .expect("Cannot set current branch name")
}
