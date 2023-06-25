use crate::db_ops::{Persistence, DB};

use super::restore_command::run_restore_command;

pub fn run_switch_command(db_path: &str, branch_name: &str, file_path: &str) {
    let db = Persistence::open(db_path).expect("Cannot open db");
    
    let tip = db
        .read_branch_tip(branch_name)
        .expect("Branch does not exist");

    db.write_current_branch_name(branch_name)
        .expect("Cannot set branch as current branch");

    run_restore_command(file_path, db_path, &tip);
}
