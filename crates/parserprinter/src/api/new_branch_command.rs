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

#[cfg(test)]
mod test {
    use tempfile::TempDir;

    use crate::{
        api::test_utils,
        db_ops::{Persistence, DB},
    };

    use super::run_new_branch_command;

    #[test]
    fn test_create_new_branch() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_db_path);

        run_new_branch_command(tmp_db_path, "dev".to_owned());

        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");
        assert_eq!(db.read_all_commits().unwrap().len(), 0);

        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");

        let branches = db.read_all_branches().unwrap();
        assert_eq!(branches, vec!["dev", "main"]);

        // the current branch name is updated to the name of the new branch
        assert_eq!(current_branch_name, "dev");

        let latest_commit_name = db
            .read_current_latest_commit()
            .expect("Cannot read latest commit");

        // the latest commit hash stays the same
        assert_eq!(latest_commit_name, "initial");
    }

    #[test]
    fn test_commit_to_new_branch() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_db_path);

        // a commit to `main`
        test_utils::commit(tmp_db_path, "Commit", "data/untitled.blend");

        run_new_branch_command(tmp_db_path, "dev".to_owned());

        // a commit to `other`
        test_utils::commit(tmp_db_path, "Commit 2", "data/untitled_2.blend");

        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");

        let commits = db.read_all_commits().unwrap();

        assert_eq!(commits.len(), 2);

        assert_eq!(
            commits
                .iter()
                .map(|c| c.message.clone())
                .collect::<Vec<String>>(),
            vec!["Commit", "Commit 2"]
        );

        // latest commit first
        assert_eq!(commits.get(0).unwrap().branch, "main");
        assert_eq!(commits.get(0).unwrap().message, "Commit");
        assert_eq!(commits.get(1).unwrap().branch, "dev");
        assert_eq!(commits.get(1).unwrap().message, "Commit 2");
    }
}
