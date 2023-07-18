use crate::db::db_ops::{Persistence, DB};

pub fn run_init_command(db_path: &str) {
    let db = Persistence::open(db_path).expect("Cannot open DB");
    db.write_branch_tip("main", "initial")
        .expect("Cannot write default branch");
    db.write_current_branch_name("main")
        .expect("Cannot write current branch");
    db.write_current_latest_commit("initial")
        .expect("Cannot write current latest commit");
}

#[cfg(test)]
mod test {
    use tempfile::TempDir;

    use crate::db::db_ops::{Persistence, DB};

    use super::run_init_command;

    #[test]
    fn test_post_init_state() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");
        run_init_command(tmp_path);

        let db = Persistence::open(tmp_path).expect("Cannot open db");
        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");
        assert_eq!(current_branch_name, "main");

        let latest_commit_name = db
            .read_current_latest_commit()
            .expect("Cannot read latest commit");
        assert_eq!(latest_commit_name, "initial");
    }
}
