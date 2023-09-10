use std::collections::HashMap;

use crate::{
    db::db_ops::{DBError, Persistence, DB},
    exchange::structs::Exchange,
};

// TODO: rename conflicting commits
pub fn import_exchange(db_path: &str, exchange: Exchange) -> Result<(), DBError> {
    let mut branches_to_commits: HashMap<String, String> = HashMap::new();

    for commit in &exchange.commits {
        branches_to_commits.insert(commit.branch.clone(), commit.hash.clone());
    }

    let mut db = Persistence::open(db_path)?;
    db.write_blocks(&exchange.blocks)?;

    for commit in &exchange.commits {
        db.write_blocks_str(&commit.hash, &commit.blocks)?;
    }

    db.execute_in_transaction(|tx| {
        for commit in exchange.commits.into_iter() {
            Persistence::write_commit(tx, commit)?;
        }

        Ok(())
    })?;

    let mut branches_to_tips: HashMap<String, String> = HashMap::new();

    for (branch, commit_hash) in branches_to_commits.into_iter() {
        let tip = db
            .read_descendants_of_commit(&commit_hash)?
            .last()
            .map(|c| c.hash.clone())
            .unwrap_or(commit_hash);
        branches_to_tips.insert(branch, tip);
    }

    db.execute_in_transaction(|tx| {
        for (branch, tip) in branches_to_tips.into_iter() {
            Persistence::write_branch_tip(tx, &branch, &tip)?;
        }

        Ok(())
    })
}

/*
test
- commits have to be present
- blocks have to be present
- branches have to be updated
 */

#[cfg(test)]
mod test {
    use tempfile::TempDir;

    use crate::{
        api::test_utils,
        db::{
            db_ops::{Persistence, DB},
            structs::{BlockRecord, Commit},
        },
        exchange::structs::Exchange,
    };

    use super::import_exchange;

    #[test]
    fn test_export_exchange() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_db_path, "my-cool-project");

        {
            let mut db = Persistence::open(tmp_db_path).expect("Cannot open test DB");

            /*
            Start:
              1 - 2 - 3

            Result:
                        x
                      /
              1 - 2 - 3 - 4
               \
                  a - b
            */

            let blocks = vec![
                BlockRecord {
                    hash: String::from("aaa"),
                    data: vec![1, 1, 1],
                },
                BlockRecord {
                    hash: String::from("bbb"),
                    data: vec![2, 2, 2],
                },
                BlockRecord {
                    hash: String::from("ccc"),
                    data: vec![3, 3, 3],
                },
                BlockRecord {
                    hash: String::from("ddd"),
                    data: vec![4, 4, 4],
                },
            ];

            db.write_blocks(&blocks).expect("cannot write blocks");

            db.write_blocks_str("1", "aaa,bbb").unwrap();
            db.write_blocks_str("2", "bbb,ccc").unwrap();
            db.write_blocks_str("3", "ccc,ddd").unwrap();

            db.execute_in_transaction(|tx| {
                Persistence::write_commit(
                    tx,
                    Commit {
                        hash: "1".to_owned(),
                        prev_commit_hash: "initial".to_owned(),
                        project_id: "a".to_owned(),
                        branch: "main".to_owned(),
                        message: "hi".to_owned(),
                        author: "test".to_owned(),
                        date: 1,
                        header: vec![],
                        blocks: "aaa,bbb".to_owned(),
                    },
                )?;

                Persistence::write_commit(
                    tx,
                    Commit {
                        hash: "2".to_owned(),
                        prev_commit_hash: "1".to_owned(),
                        project_id: "a".to_owned(),
                        branch: "main".to_owned(),
                        message: "hi".to_owned(),
                        author: "test".to_owned(),
                        date: 2,
                        header: vec![],
                        blocks: "bbb,ccc".to_owned(),
                    },
                )?;

                Persistence::write_commit(
                    tx,
                    Commit {
                        hash: "3".to_owned(),
                        prev_commit_hash: "2".to_owned(),
                        project_id: "a".to_owned(),
                        branch: "main".to_owned(),
                        message: "hi".to_owned(),
                        author: "test".to_owned(),
                        date: 3,
                        header: vec![],
                        blocks: "ccc,ddd".to_owned(),
                    },
                )?;

                Ok(())
            })
            .expect("Cannot execute transaction");
        }

        let exchange = Exchange {
            commits: vec![
                Commit {
                    hash: "4".to_owned(),
                    prev_commit_hash: "3".to_owned(),
                    project_id: "a".to_owned(),
                    branch: "main".to_owned(),
                    message: "hi".to_owned(),
                    author: "test".to_owned(),
                    date: 4,
                    header: vec![],
                    blocks: "ddd,eee".to_owned(),
                },
                Commit {
                    hash: "a".to_owned(),
                    prev_commit_hash: "1".to_owned(),
                    project_id: "a".to_owned(),
                    branch: "ab".to_owned(),
                    message: "hi".to_owned(),
                    author: "test".to_owned(),
                    date: 10,
                    header: vec![],
                    blocks: "eee,fff".to_owned(),
                },
                Commit {
                    hash: "b".to_owned(),
                    prev_commit_hash: "a".to_owned(),
                    project_id: "a".to_owned(),
                    branch: "ab".to_owned(),
                    message: "hi".to_owned(),
                    author: "test".to_owned(),
                    date: 11,
                    header: vec![],
                    blocks: "fff,111".to_owned(),
                },
                Commit {
                    hash: "x".to_owned(),
                    prev_commit_hash: "3".to_owned(),
                    project_id: "a".to_owned(),
                    branch: "xs".to_owned(),
                    message: "hi".to_owned(),
                    author: "test".to_owned(),
                    date: 10,
                    header: vec![],
                    blocks: "222,aaa".to_owned(),
                },
            ],
            blocks: vec![
                BlockRecord {
                    hash: String::from("aaa"),
                    data: vec![1, 1, 1],
                },
                BlockRecord {
                    hash: String::from("ddd"),
                    data: vec![4, 4, 4],
                },
                BlockRecord {
                    hash: String::from("eee"),
                    data: vec![5, 5, 5],
                },
                BlockRecord {
                    hash: String::from("fff"),
                    data: vec![6, 6, 6],
                },
                BlockRecord {
                    hash: String::from("111"),
                    data: vec![7, 7, 7],
                },
                BlockRecord {
                    hash: String::from("222"),
                    data: vec![7, 7, 7],
                },
            ],
        };

        import_exchange(tmp_db_path, exchange).expect("Cannot import exchange");
        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");

        let all_commits = db
            .read_descendants_of_commit("1")
            .expect("Cannot read descendants of 1");

        let all_commit_hashes: Vec<String> = all_commits.iter().map(|c| c.hash.clone()).collect();

        assert_eq!(all_commit_hashes, vec!["1", "2", "3", "4", "a", "x", "b"]); // all commits are present

        let mut all_branches: Vec<String> = all_commits.iter().map(|c| c.branch.clone()).collect();

        // poor man's `unique`
        all_branches.sort_unstable();
        all_branches.dedup();

        assert_eq!(all_branches, vec!["ab", "main", "xs"]);

        let ab_tip = db
            .read_branch_tip("ab")
            .expect("Cannot read tip for branch 'ab'")
            .expect("Branch 'ab' should have a tip");
        assert_eq!(ab_tip, "b");

        let main_tip = db
            .read_branch_tip("main")
            .expect("Cannot read tip for branch 'main'")
            .expect("Branch 'main' should have a tip");
        assert_eq!(main_tip, "4");

        let main_tip = db
            .read_branch_tip("xs")
            .expect("Cannot read tip for branch 'xs'")
            .expect("Branch 'xs' should have a tip");
        assert_eq!(main_tip, "x");
    }
}
