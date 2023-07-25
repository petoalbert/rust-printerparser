use std::{collections::HashMap, path::Path};

use crate::{exchange::Exchange, printer_parser::printerparser::PrinterParser};

use super::structs::{hash_list, BlockRecord, Commit};

type Connection = rusqlite::Connection;

pub struct ShortCommitRecord {
    pub hash: String,
    pub branch: String,
    pub message: String,
}

pub struct ExportResult {
    pub exchange: Exchange,
    pub skipped: Vec<String>,
}

#[derive(Debug)]
pub struct DBError(pub String);

pub trait DB: Sized {
    fn open(path: &str) -> Result<Self, DBError>;

    fn read_config(&self, key: &str) -> Result<Option<String>, DBError>;
    fn write_config(&self, key: &str, value: &str) -> Result<(), DBError>;

    fn write_blocks(&self, blocks: &[BlockRecord]) -> Result<(), DBError>;
    fn read_blocks(&self, hashes: Vec<String>) -> Result<Vec<BlockRecord>, DBError>;

    fn write_commit(&self, commit: Commit) -> Result<(), DBError>;
    fn read_commit(&self, hash: &str) -> Result<Option<Commit>, DBError>;

    fn read_commits_for_branch(&self, brach_name: &str) -> Result<Vec<ShortCommitRecord>, DBError>;
    fn read_all_commits(&self) -> Result<Vec<ShortCommitRecord>, DBError>;

    fn read_current_branch_name(&self) -> Result<String, DBError>;
    fn write_current_branch_name(&self, brach_name: &str) -> Result<(), DBError>;

    fn read_current_latest_commit(&self) -> Result<String, DBError>;
    fn write_current_latest_commit(&self, hash: &str) -> Result<(), DBError>;

    fn read_all_branches(&self) -> Result<Vec<String>, DBError>;

    fn read_branch_tip(&self, branch_name: &str) -> Result<Option<String>, DBError>;
    fn write_branch_tip(&self, brach_name: &str, tip: &str) -> Result<(), DBError>;

    fn export_commits(&self, commits: Vec<String>) -> Result<ExportResult, DBError>;
    fn import_exchange(&self, exchange: Exchange) -> Result<(), DBError>;
}

pub struct Persistence {
    rocks_db: rocksdb::DB,
    sqlite_db: rusqlite::Connection,
}

#[inline]
fn block_hash_key(key: &str) -> String {
    format!("block-hash-{:?}", key)
}

#[inline]
fn working_dir_key(key: &str) -> String {
    format!("working-dir-{:?}", key)
}

#[inline]
fn current_branch_name_key() -> String {
    "CURRENT_BRANCH_NAME".to_string()
}

#[inline]
fn current_latest_commit_key() -> String {
    "CURRENT_LATEST_COMMIT".to_string()
}

impl DB for Persistence {
    fn open(path: &str) -> Result<Self, DBError> {
        let sqlite_path = Path::new(path).join("commits.sqlite");
        let rocks_path = Path::new(path).join("blobs.rocks");

        let rocks_db = rocksdb::DB::open_default(rocks_path).expect("Cannot open rocksdb");
        let sqlite_db = Connection::open(sqlite_path).expect("Cannot open sqlite db");

        sqlite_db
            .execute(
                "CREATE TABLE IF NOT EXISTS commits (
                    hash TEXT PRIMARY KEY,
                    prev_commit_hash TEXT,
                    branch TEXT,
                    message TEXT,
                    author TEXT,
                    date INTEGER,
                    header BLOB
                )",
                [],
            )
            .expect("Cannot create commits table");

        sqlite_db
            .execute(
                "CREATE TABLE IF NOT EXISTS branches (
                    name TEXT PRIMARY KEY,
                    tip TEXT
                )",
                [],
            )
            .expect("Cannot create branches table");

        sqlite_db
            .execute(
                "CREATE TABLE IF NOT EXISTS config (
                    key TEXT PRIMARY KEY,
                    value TEXT
                )",
                [],
            )
            .expect("Cannot create config table");

        Ok(Self {
            rocks_db,
            sqlite_db,
        })
    }

    fn read_config(&self, key: &str) -> Result<Option<String>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT value FROM config WHERE key = ?1")
            .map_err(|_| DBError("Cannot prepare read commits query".to_owned()))?;

        let mut rows = stmt
            .query([key])
            .map_err(|_| DBError("Cannot query config table".to_owned()))?;

        match rows.next() {
            Ok(Some(row)) => row
                .get(0)
                .map_err(|_| DBError("Cannot read config key".to_owned())),
            _ => Ok(None),
        }
    }

    fn write_config(&self, key: &str, value: &str) -> Result<(), DBError> {
        self.sqlite_db
            .execute(
                "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
                [key, value],
            )
            .map_err(|_| DBError(format!("Cannot set {:?} for {:?}", value, key)))
            .map(|_| ())
    }

    fn write_blocks(&self, blocks: &[BlockRecord]) -> Result<(), DBError> {
        for block in blocks {
            self.rocks_db
                .put(block_hash_key(&block.hash), &block.data)
                .expect("Cannot write block");
        }

        Ok(())
    }

    fn read_blocks(&self, hashes: Vec<String>) -> Result<Vec<BlockRecord>, DBError> {
        let mut result: Vec<BlockRecord> = Vec::new();
        for hash in hashes {
            let block_data = self
                .rocks_db
                .get(block_hash_key(&hash))
                .expect("Error reading block")
                .expect("No block with hash found");
            result.push(BlockRecord {
                hash,
                data: block_data,
            })
        }

        Ok(result)
    }

    fn write_commit(&self, commit: Commit) -> Result<(), DBError> {
        self.rocks_db
            .put(working_dir_key(&commit.hash), commit.blocks)
            .expect("Cannot write working dir blocks");

        self.sqlite_db.execute(
            "INSERT INTO commits (hash, prev_commit_hash, branch, message, author, date, header) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                commit.hash,
                commit.prev_commit_hash,
                commit.branch,
                commit.message,
                commit.author,
                commit.date,
                commit.header,
            ),
        ).expect("Cannot insert commit object");

        Ok(())
    }

    fn read_commit(&self, hash: &str) -> Result<Option<Commit>, DBError> {
        let blocks = self
            .rocks_db
            .get(working_dir_key(hash))
            .expect("Cannot read working dir key")
            .map(|bs| String::from_utf8(bs).unwrap())
            .expect("No working dir found");

        self.sqlite_db.query_row("SELECT hash, prev_commit_hash, branch, message, author, date, header FROM commits WHERE hash = ?1", [hash], |row| Ok(Some(Commit {
            hash: row.get(0).expect("No hash found in row"),
            prev_commit_hash: row.get(1).expect("No prev_commit_hash found in row"),
            branch: row.get(2).expect("No branch found in row"),
            message: row.get(3).expect("No message found in row"),
            author: row.get(4).expect("No author found in row"),
            date: row.get(5).expect("No date found in row"),
            header: row.get(6).expect("No header found in row"),
            blocks,
        }))).map_err(|e| DBError(format!("Cannot read commit: {:?}", e)))
    }

    fn read_all_commits(&self) -> Result<Vec<ShortCommitRecord>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT hash, branch, message FROM commits ORDER BY date DESC")
            .expect("Cannot prepare read commits query");

        let mut rows = stmt.query([]).expect("cannot read commits");

        let mut result: Vec<ShortCommitRecord> = vec![];
        while let Ok(Some(data)) = rows.next() {
            result.push(ShortCommitRecord {
                hash: data.get(0).expect("cannot get hash"),
                branch: data.get(1).expect("cannot get branch"),
                message: data.get(2).expect("cannot read message"),
            })
        }

        Ok(result)
    }

    fn read_commits_for_branch(&self, brach_name: &str) -> Result<Vec<ShortCommitRecord>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare(
                "SELECT hash, branch, message FROM commits WHERE branch = ?1 ORDER BY date DESC",
            )
            .expect("Cannot prepare read commits query");

        let mut rows = stmt.query([brach_name]).expect("cannot read commits");

        let mut result: Vec<ShortCommitRecord> = vec![];
        while let Ok(Some(data)) = rows.next() {
            result.push(ShortCommitRecord {
                hash: data.get(0).expect("cannot get hash"),
                branch: data.get(1).expect("cannot get branch"),
                message: data.get(2).expect("cannot read message"),
            })
        }

        Ok(result)
    }

    fn read_current_branch_name(&self) -> Result<String, DBError> {
        self.read_config(&current_branch_name_key())
            .map_err(|_| DBError("Cannot read current branch name".to_owned()))
            .and_then(|v| {
                v.map_or(
                    Err(DBError("Cannot read current branch name".to_owned())),
                    Ok,
                )
            })
    }

    fn write_current_branch_name(&self, brach_name: &str) -> Result<(), DBError> {
        self.write_config(&current_branch_name_key(), brach_name)
    }

    fn read_all_branches(&self) -> Result<Vec<String>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT name FROM branches")
            .map_err(|e| DBError(format!("Cannot query branches: {:?}", e)))?;
        let mut rows = stmt
            .query([])
            .map_err(|e| DBError(format!("Cannot query branches: {:?}", e)))?;

        let mut result: Vec<String> = vec![];

        while let Ok(Some(data)) = rows.next() {
            result.push(data.get(0).expect("cannot get branch name"))
        }

        Ok(result)
    }

    fn read_branch_tip(&self, branch_name: &str) -> Result<Option<String>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT tip FROM branches WHERE name = ?1")
            .map_err(|e| DBError(format!("Cannot query branch: {:?}", e)))?;

        let mut rows = stmt
            .query([branch_name])
            .map_err(|e| DBError(format!("Cannot query branch: {:?}", e)))?;

        let row = rows.next();

        if let Ok(Some(data)) = row {
            Ok(Some(data.get(0).unwrap()))
        } else if let Ok(None) = row {
            Ok(None)
        } else {
            Err(DBError("Cannot query branch".to_owned()))
        }
    }

    fn write_branch_tip(&self, brach_name: &str, tip: &str) -> Result<(), DBError> {
        self.sqlite_db
            .execute(
                "INSERT OR REPLACE INTO branches (name, tip) VALUES (?1, ?2)",
                [&brach_name, &tip],
            )
            .map_err(|e| {
                DBError(format!(
                    "Cannot create new branch {:?}: {:?}",
                    brach_name, e
                ))
            })
            .map(|_| ())
    }

    fn read_current_latest_commit(&self) -> Result<String, DBError> {
        self.read_config(&current_latest_commit_key())
            .map_err(|_| DBError("Cannot read latest commit hash".to_owned()))
            .and_then(|v| {
                v.map_or(
                    Err(DBError("Cannot read latest commit hash".to_owned())),
                    Ok,
                )
            })
    }

    fn write_current_latest_commit(&self, hash: &str) -> Result<(), DBError> {
        self.write_config(&current_latest_commit_key(), hash)
            .map_err(|e| DBError(format!("Cannot write latest commit hash: {:?}", e)))
    }

    fn export_commits(&self, hashes: Vec<String>) -> Result<ExportResult, DBError> {
        let mut blobs: HashMap<String, Vec<u8>> = HashMap::new();
        let mut commits: Vec<Commit> = vec![];
        let mut skipped: Vec<String> = vec![];

        for hash in hashes {
            let commit = self.read_commit(&hash)?;
            if let Some(commit) = commit {
                let hashes = hash_list()
                    .parse(&commit.blocks, &mut ())
                    .map_err(|_| DBError("Cannot parse blocks".to_owned()))?
                    .1;

                commits.push(commit);

                let blocks = self.read_blocks(hashes)?;

                for block in blocks {
                    blobs.insert(block.hash, block.data);
                }
            } else {
                skipped.push(hash)
            }
        }

        let blocks: Vec<BlockRecord> = blobs
            .into_iter()
            .map(|(hash, data)| BlockRecord { hash, data })
            .collect();

        Ok(ExportResult {
            exchange: Exchange { commits, blocks },
            skipped,
        })
    }

    fn import_exchange(&self, exchange: Exchange) -> Result<(), DBError> {
        let mut new_branches: HashMap<String, (u64, String)> = HashMap::new();
        for commit in exchange.commits {
            let commit_hash = commit.hash.clone();
            let commit_date = commit.date;
            let commit_branch = commit.branch.clone();

            self.write_commit(commit)?;

            let latest = new_branches.get(&commit_branch);
            match latest {
                None => {
                    new_branches.insert(commit_branch, (commit_date, commit_hash));
                }
                Some((date, _)) if commit_date > (*date) => {
                    new_branches.insert(commit_branch, (commit_date, commit_hash));
                }
                _ => {}
            }
        }

        for (branch, (_, tip)) in new_branches {
            self.write_branch_tip(&branch, &tip)?;
        }

        self.write_blocks(&exchange.blocks)?;

        Ok(())
    }
}
