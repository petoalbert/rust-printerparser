use std::collections::HashSet;

use crate::{
    db::{
        db_ops::{DBError, Persistence, DB},
        structs::hash_list,
    },
    exchange::structs::Exchange,
    printer_parser::printerparser::PrinterParser,
};

pub fn export_descendants_of_commit(
    db_path: &str,
    starting_from_commit_hash: &str,
) -> Result<Exchange, DBError> {
    let db = Persistence::open(db_path)?;
    let commits = db.read_descendants_of_commit(starting_from_commit_hash)?;
    let mut block_hashes: HashSet<String> = HashSet::new();

    for commit in commits.iter() {
        let blocks_of_this_commit = hash_list()
            .parse(&commit.blocks, &mut ())
            .expect("Corrupted hash list")
            .1;

        for block in blocks_of_this_commit.into_iter() {
            if !block_hashes.contains(&block) {
                block_hashes.insert(block);
            }
        }
    }

    let blocks = db.read_blocks(block_hashes.into_iter().collect())?;

    Ok(Exchange { commits, blocks })
}

#[cfg(test)]
mod test {
    use tempfile::TempDir;

    use crate::{
        api::test_utils,
        db::{
            db_ops::{Persistence, DB},
            structs::{BlockRecord, Commit},
        },
    };

    use super::export_descendants_of_commit;

    #[test]
    fn test_export_exchange() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_path, "my-cool-project");

        {
            let mut db = Persistence::open(tmp_path).expect("Cannot open test DB");

            /*
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
                BlockRecord {
                    hash: String::from("eee"),
                    data: vec![5, 5, 5],
                },
                BlockRecord {
                    hash: String::from("fff"),
                    data: vec![6, 6, 6],
                },
                BlockRecord {
                    hash: String::from("222"),
                    data: vec![7, 7, 7],
                },
            ];

            db.write_blocks(&blocks).expect("cannot write blocks");

            db.write_blocks_str("1", "aaa,bbb").unwrap();
            db.write_blocks_str("2", "bbb,ccc").unwrap();
            db.write_blocks_str("3", "ccc,ddd").unwrap();
            db.write_blocks_str("4", "ddd,eee").unwrap();
            db.write_blocks_str("a", "eee,fff").unwrap();
            db.write_blocks_str("b", "fff,111").unwrap();
            db.write_blocks_str("x", "222,aaa").unwrap();
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

                Persistence::write_commit(
                    tx,
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
                )?;

                Persistence::write_commit(
                    tx,
                    Commit {
                        hash: "a".to_owned(),
                        prev_commit_hash: "1".to_owned(),
                        project_id: "a".to_owned(),
                        branch: "main".to_owned(),
                        message: "hi".to_owned(),
                        author: "test".to_owned(),
                        date: 10,
                        header: vec![],
                        blocks: "eee,fff".to_owned(),
                    },
                )?;

                Persistence::write_commit(
                    tx,
                    Commit {
                        hash: "b".to_owned(),
                        prev_commit_hash: "a".to_owned(),
                        project_id: "a".to_owned(),
                        branch: "main".to_owned(),
                        message: "hi".to_owned(),
                        author: "test".to_owned(),
                        date: 11,
                        header: vec![],
                        blocks: "fff,111".to_owned(),
                    },
                )?;

                Persistence::write_commit(
                    tx,
                    Commit {
                        hash: "x".to_owned(),
                        prev_commit_hash: "3".to_owned(),
                        project_id: "a".to_owned(),
                        branch: "main".to_owned(),
                        message: "hi".to_owned(),
                        author: "test".to_owned(),
                        date: 10,
                        header: vec![],
                        blocks: "222,aaa".to_owned(),
                    },
                )?;

                Ok(())
            })
            .expect("Cannot execute transaction");
        }

        let exchange = export_descendants_of_commit(tmp_path, "3").expect("Cannot export commits");
        assert_eq!(exchange.commits.len(), 3);
        assert_eq!(exchange.blocks.len(), 5);
        assert_eq!(exchange.commits.get(0).unwrap().hash, "3");
        assert_eq!(exchange.commits.get(1).unwrap().hash, "4");
        assert_eq!(exchange.commits.get(2).unwrap().hash, "x");

        let mut hashes: Vec<String> = exchange.blocks.into_iter().map(|b| b.hash).collect();
        hashes.sort();

        assert_eq!(hashes, vec!["222", "aaa", "ccc", "ddd", "eee"]);
    }
}
