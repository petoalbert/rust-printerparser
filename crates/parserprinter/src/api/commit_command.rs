use crate::{
    api::{
        common::{blend_file_data_from_file, read_latest_commit_hash_on_branch},
        utils::{block_hash_diff, timestamp},
    },
    db::{
        db_ops::{DBError, Persistence, DB},
        structs::{hash_list, Commit},
    },
    measure_time,
    printer_parser::printerparser::PrinterParser,
};

use std::time::Instant;

pub fn create_new_commit(
    file_path: &str,
    db_path: &str,
    message: Option<String>,
) -> Result<(), DBError> {
    let mut conn = Persistence::open(db_path)?;

    let start_commit_command = Instant::now();
    let blend_data = blend_file_data_from_file(file_path)
        .map_err(|e| DBError::Error(format!("Error parsing blend file: {}", e)))?;

    println!("Hash: {}", &blend_data.hash);

    let current_branch_name = conn.read_current_branch_name()?;

    let latest_commit_hash = read_latest_commit_hash_on_branch(&conn, &current_branch_name)?;

    let latest_commit = conn.read_commit(&latest_commit_hash).ok().flatten();

    let blocks_from_latest = match latest_commit {
        None => blend_data.block_data,
        Some(commit) => {
            let hashes = hash_list().parse(&commit.blocks, &mut ()).unwrap().1;
            block_hash_diff(hashes, blend_data.block_data)
        }
    };

    measure_time!(format!("Writing blocks {:?}", file_path), {
        conn.write_blocks(&blocks_from_latest[..])?
    });

    let project_id = conn.read_project_id()?;

    let name = conn.read_name()?.unwrap_or("Anon".to_owned());

    conn.write_blocks_str(&blend_data.hash, &blend_data.blocks)?;

    conn.execute_in_transaction(|tx| {
        Persistence::write_branch_tip(tx, &current_branch_name, &blend_data.hash)?;

        let commit = Commit {
            hash: blend_data.hash,
            prev_commit_hash: latest_commit_hash,
            project_id,
            branch: current_branch_name,
            message: message.unwrap_or_default(),
            author: name,
            date: timestamp(),
            header: blend_data.header_bytes,
            blocks: blend_data.blocks,
        };

        Persistence::write_commit(tx, commit)
    })?;

    println!("Committing took {:?}", start_commit_command.elapsed());
    Ok(())
}

#[cfg(test)]
mod test {
    use tempfile::TempDir;

    use crate::{
        api::{
            common::read_latest_commit_hash_on_branch, init_command::MAIN_BRANCH_NAME, test_utils,
        },
        db::db_ops::{Persistence, DB},
    };

    use super::create_new_commit;

    #[test]
    fn test_initial_commit() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db_from_file(tmp_path, "my-cool-project", "data/untitled.blend");

        // Creates exactly one commit
        assert_eq!(
            test_utils::list_checkpoints(tmp_path, MAIN_BRANCH_NAME).len(),
            1
        );

        create_new_commit(
            "data/untitled_2.blend",
            tmp_path,
            Some("Initial checkpoint".to_owned()),
        )
        .unwrap();

        // Creates exactly one commit
        assert_eq!(
            test_utils::list_checkpoints(tmp_path, MAIN_BRANCH_NAME).len(),
            2
        );

        let db = Persistence::open(tmp_path).expect("Cannot open test DB");

        let commit = db
            .read_commit("b637ec695e10bed0ce06279d1dc46717")
            .unwrap()
            .unwrap();

        // commit.blocks omitted, too long
        // commit.date omitted, not stable
        // commit.header omitted, not interesting enough
        assert_eq!(commit.author, "Anon");
        assert_eq!(commit.branch, MAIN_BRANCH_NAME);
        assert_eq!(commit.hash, "b637ec695e10bed0ce06279d1dc46717");
        assert_eq!(commit.message, "Initial checkpoint");
        assert_eq!(commit.prev_commit_hash, "a5f92d0a988085ed66c9dcdccc7b9c90");
        assert_eq!(commit.project_id, "my-cool-project");

        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");

        // The current branch name stays the same
        assert_eq!(current_branch_name, MAIN_BRANCH_NAME);

        let latest_commit_hash = read_latest_commit_hash_on_branch(&db, &current_branch_name)
            .expect("Cannot read latest commit");

        // The latest commit hash is updated to the hash of the new commit
        assert_eq!(latest_commit_hash, "b637ec695e10bed0ce06279d1dc46717");

        // The tip of `main` is updated to the hash of the new commit
        let main_tip = db.read_branch_tip(MAIN_BRANCH_NAME).unwrap().unwrap();
        assert_eq!(main_tip, "b637ec695e10bed0ce06279d1dc46717");
    }

    #[test]
    fn test_next_commit() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db_from_file(tmp_path, "my-cool-project", "data/untitled.blend");

        create_new_commit(
            "data/untitled_2.blend",
            tmp_path,
            Some("Message".to_owned()),
        )
        .unwrap();
        create_new_commit(
            "data/untitled_3.blend",
            tmp_path,
            Some("Message".to_owned()),
        )
        .unwrap();

        assert_eq!(
            test_utils::list_checkpoints(tmp_path, MAIN_BRANCH_NAME).len(),
            3
        );

        let db = Persistence::open(tmp_path).expect("Cannot open test DB");

        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");

        // The current branch name stays the same
        assert_eq!(current_branch_name, MAIN_BRANCH_NAME);

        let latest_commit_hash = read_latest_commit_hash_on_branch(&db, &current_branch_name)
            .expect("Cannot read latest commit");

        // The latest commit hash is updated to the hash of the new commit
        assert_eq!(latest_commit_hash, "d9e8eb09f8270ad5326de946d951433a");

        // The tip of `main` is updated to the hash of the new commit
        let main_tip = db.read_branch_tip(MAIN_BRANCH_NAME).unwrap().unwrap();
        assert_eq!(main_tip, "d9e8eb09f8270ad5326de946d951433a");
    }
}
