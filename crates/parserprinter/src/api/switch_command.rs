use crate::db::db_ops::{DBError, Persistence, DB};

use super::restore_command::restore_checkpoint;

pub fn switch_branches(db_path: &str, branch_name: &str, file_path: &str) -> Result<(), DBError> {
    let hash = {
        let mut db = Persistence::open(db_path)?;

        let tip = db.read_branch_tip(branch_name)?;

        if tip.is_none() {
            return Err(DBError::Consistency(
                "Branch has no corresponding tip".to_owned(),
            ));
        }

        let hash = tip.unwrap();

        db.execute_in_transaction(|tx| {
            Persistence::write_current_branch_name(tx, branch_name)?;
            Ok(())
        })?;

        hash
    };

    restore_checkpoint(file_path, db_path, &hash)
}

#[cfg(test)]
mod test {
    use tempfile::{NamedTempFile, TempDir};

    use crate::{
        api::{
            common::read_latest_commit_hash_on_branch, init_command::MAIN_BRANCH_NAME, test_utils,
        },
        db::db_ops::{Persistence, DB},
    };

    use super::switch_branches;

    #[test]
    fn test_checkout_non_existent_branch() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db_from_file(tmp_db_path, "my-cool-project", "data/untitled.blend");

        let res = switch_branches(tmp_db_path, "unknown", "void.blend");
        assert!(matches!(res, Err(_)));

        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");

        let branches = db.read_all_branches().unwrap();

        // no new branch is added
        assert_eq!(branches, vec!["main"]);

        let main_tip = db.read_branch_tip(MAIN_BRANCH_NAME).unwrap().unwrap();

        // tip of main stays the same
        assert_eq!(main_tip, "a5f92d0a988085ed66c9dcdccc7b9c90");

        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");

        // The current branch name stays the same
        assert_eq!(current_branch_name, "main");

        let latest_commit_hash = read_latest_commit_hash_on_branch(&db, &current_branch_name)
            .expect("Cannot read latest commit");

        // The latest commit hash stays the same
        assert_eq!(latest_commit_hash, "a5f92d0a988085ed66c9dcdccc7b9c90");
    }

    #[test]
    fn test_checkout_real_branch() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db_from_file(tmp_db_path, "my-cool-project", "data/untitled.blend");

        // a commit to `main`
        test_utils::commit(tmp_db_path, "Commit", "data/untitled_2.blend");

        test_utils::new_branch(tmp_db_path, "dev");

        // a commit to `dev`
        test_utils::commit(tmp_db_path, "Commit 2", "data/untitled_3.blend");

        let tmp_blend_path = NamedTempFile::new().expect("Cannot create temp file");

        switch_branches(
            tmp_db_path,
            MAIN_BRANCH_NAME,
            tmp_blend_path.path().to_str().unwrap(),
        )
        .expect("Cannot switch branches");

        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");

        // current branch name is set to the checked out branch
        let current_branch_name = db.read_current_branch_name().unwrap();
        assert_eq!(current_branch_name, MAIN_BRANCH_NAME);

        // latest commit hash is set to the tip of the checked out branch
        let lastest_commit_hash =
            read_latest_commit_hash_on_branch(&db, &current_branch_name).unwrap();
        assert_eq!(lastest_commit_hash, "b637ec695e10bed0ce06279d1dc46717");

        let main_tip = db.read_branch_tip(MAIN_BRANCH_NAME).unwrap().unwrap();
        assert_eq!(lastest_commit_hash, main_tip);
    }
}
