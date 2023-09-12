use crate::db::db_ops::{DBError, Persistence, DB};

pub fn init_db(db_path: &str, project_id: &str) -> Result<(), DBError> {
    let mut db = Persistence::open(db_path)?;
    db.execute_in_transaction(|tx| {
        Persistence::write_branch_tip(tx, "main", "initial")?;
        Persistence::write_remote_branch_tip(tx, "main", "initial")?;
        Persistence::write_current_branch_name(tx, "main")?;
        Persistence::write_current_latest_commit(tx, "initial")?;
        Persistence::write_project_id(tx, project_id)?;
        Ok(())
    })?;
    Ok(())
}

#[cfg(test)]
mod test {
    use tempfile::TempDir;

    use crate::db::db_ops::{Persistence, DB};

    use super::init_db;

    #[test]
    fn test_post_init_state() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");
        init_db(tmp_path, "my amazing project").expect("Cannot init DB");

        let db = Persistence::open(tmp_path).expect("Cannot open db");
        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");
        assert_eq!(current_branch_name, "main");

        let latest_commit_name = db
            .read_current_latest_commit()
            .expect("Cannot read latest commit");
        assert_eq!(latest_commit_name, "initial");

        let project_id = db.read_project_id().expect("Cannot read project id");
        assert_eq!(project_id, "my amazing project")
    }
}
