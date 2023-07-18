use crate::db::db_ops::{DBError, Persistence, DB};

use super::restore_command::restore_checkpoint;

pub fn switch_branches(db_path: &str, branch_name: &str, file_path: &str) -> Result<(), DBError> {
    let hash = {
        let db = Persistence::open(db_path).expect("Cannot open db");

        let tip = db.read_branch_tip(branch_name)?;

        if tip.is_none() {
            return Ok(()); // FIXME
        }

        let hash = tip.unwrap();

        db.write_current_branch_name(branch_name)?;

        db.write_current_latest_commit(&hash)?;

        hash
    };

    restore_checkpoint(file_path, db_path, &hash)
}

#[cfg(test)]
mod test {
    use tempfile::{NamedTempFile, TempDir};

    use crate::{
        api::test_utils,
        db::db_ops::{Persistence, DB},
    };

    use super::switch_branches;

    #[test]
    fn test_checkout_non_existent_branch() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_db_path);

        switch_branches(tmp_db_path, "unknown", "void.blend").expect("Cannotttt switch branches");

        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");
        assert_eq!(db.read_all_commits().unwrap().len(), 0);

        let branches = db.read_all_branches().unwrap();

        // no new branch is added
        assert_eq!(branches, vec!["main"]);

        let main_tip = db.read_branch_tip("main").unwrap().unwrap();

        // tip of main stays the same
        assert_eq!(main_tip, "initial");

        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");

        // The current branch name stays the same
        assert_eq!(current_branch_name, "main");

        let latest_commit_hash = db
            .read_current_latest_commit()
            .expect("Cannot read latest commit");

        // The latest commit hash stays the same
        assert_eq!(latest_commit_hash, "initial");
    }

    #[test]
    fn test_checkout_real_branch() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_db_path);

        // a commit to `main`
        test_utils::commit(tmp_db_path, "Commit", "data/untitled.blend");

        test_utils::new_branch(tmp_db_path, "dev");

        // a commit to `dev`
        test_utils::commit(tmp_db_path, "Commit 2", "data/untitled_2.blend");

        let tmp_blend_path = NamedTempFile::new().expect("Cannot create temp file");

        switch_branches(tmp_db_path, "main", tmp_blend_path.path().to_str().unwrap())
            .expect("Cannot switch branches");

        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");

        // current branch name is set to the checked out branch
        let current_branch_name = db.read_current_branch_name().unwrap();
        assert_eq!(current_branch_name, "main");

        // latest commit hash is set to the tip of the checked out branch
        let lastest_commit_hash = db.read_current_latest_commit().unwrap();
        assert_eq!(lastest_commit_hash, "a5f92d0a988085ed66c9dcdccc7b9c90");

        let main_tip = db.read_branch_tip("main").unwrap().unwrap();
        assert_eq!(lastest_commit_hash, main_tip);
    }
}
