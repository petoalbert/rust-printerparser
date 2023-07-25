use std::{fmt::Display, path::Path};

use crate::exchange::Exchange;

use super::structs::{BlockRecord, Commit};

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
pub enum DBError {
    Fundamental(String), // means that stuff is very wrong
    Consistency(String), // the timeline maybe in an inconsistent state
    Error(String),       // a recoverable error
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::Fundamental(msg) => write!(f, "Fundamental error: {}", msg),
            DBError::Consistency(msg) => write!(f, "Consistency error: {}", msg),
            DBError::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

pub trait DB: Sized {
    fn open(path: &str) -> Result<Self, DBError>;

    fn read_config(&self, key: &str) -> Result<Option<String>, DBError>;
    fn write_config(tx: &rusqlite::Transaction, key: &str, value: &str) -> Result<(), DBError>;

    fn write_blocks(&self, blocks: &[BlockRecord]) -> Result<(), DBError>;
    fn read_blocks(&self, hashes: Vec<String>) -> Result<Vec<BlockRecord>, DBError>;

    fn write_commit(tx: &rusqlite::Transaction, commit: Commit) -> Result<(), DBError>;
    fn write_blocks_str(&self, hash: &str, blocks_str: &str) -> Result<(), DBError>;
    fn read_commit(&self, hash: &str) -> Result<Option<Commit>, DBError>;

    fn read_commits_for_branch(&self, brach_name: &str) -> Result<Vec<ShortCommitRecord>, DBError>;
    fn read_all_commits(&self) -> Result<Vec<ShortCommitRecord>, DBError>;

    fn read_current_branch_name(&self) -> Result<String, DBError>;
    fn write_current_branch_name(
        tx: &rusqlite::Transaction,
        brach_name: &str,
    ) -> Result<(), DBError>;

    fn read_current_latest_commit(&self) -> Result<String, DBError>;
    fn write_current_latest_commit(tx: &rusqlite::Transaction, hash: &str) -> Result<(), DBError>;

    fn read_all_branches(&self) -> Result<Vec<String>, DBError>;

    fn read_branch_tip(&self, branch_name: &str) -> Result<Option<String>, DBError>;
    fn write_branch_tip(
        tx: &rusqlite::Transaction,
        brach_name: &str,
        tip: &str,
    ) -> Result<(), DBError>;

    fn execute_in_transaction<F>(&mut self, f: F) -> Result<(), DBError>
    where
        F: FnOnce(&rusqlite::Transaction) -> Result<(), DBError>;
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

fn write_config_inner(tx: &rusqlite::Transaction, key: &str, value: &str) -> Result<(), DBError> {
    tx.execute(
        "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
        [key, value],
    )
    .map_err(|_| DBError::Error(format!("Cannot set {:?} for {:?}", value, key)))
    .map(|_| ())
}

impl DB for Persistence {
    fn open(path: &str) -> Result<Self, DBError> {
        let sqlite_path = Path::new(path).join("commits.sqlite");
        let rocks_path = Path::new(path).join("blobs.rocks");

        let rocks_db = rocksdb::DB::open_default(rocks_path)
            .map_err(|e| DBError::Fundamental(format!("Cannot open RocksDB: {:?}", e)))?;
        let sqlite_db = rusqlite::Connection::open(sqlite_path)
            .map_err(|e| DBError::Fundamental(format!("Cannot open SQLite: {:?}", e)))?;

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
            .map_err(|e| DBError::Fundamental(format!("Cannot create commits table: {:?}", e)))?;

        sqlite_db
            .execute(
                "CREATE TABLE IF NOT EXISTS branches (
                    name TEXT PRIMARY KEY,
                    tip TEXT
                )",
                [],
            )
            .map_err(|e| DBError::Fundamental(format!("Cannot create branches table: {:?}", e)))?;

        sqlite_db
            .execute(
                "CREATE TABLE IF NOT EXISTS config (
                    key TEXT PRIMARY KEY,
                    value TEXT
                )",
                [],
            )
            .map_err(|e| DBError::Fundamental(format!("Cannot create config table: {:?}", e)))?;

        Ok(Self {
            rocks_db,
            sqlite_db,
        })
    }

    fn read_config(&self, key: &str) -> Result<Option<String>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT value FROM config WHERE key = ?1")
            .map_err(|_| DBError::Fundamental("Cannot prepare read commits query".to_owned()))?;

        let mut rows = stmt
            .query([key])
            .map_err(|_| DBError::Fundamental("Cannot query config table".to_owned()))?;

        match rows.next() {
            Ok(Some(row)) => row
                .get(0)
                .map_err(|_| DBError::Fundamental("Cannot read config key".to_owned())),
            _ => Ok(None),
        }
    }

    fn write_config(tx: &rusqlite::Transaction, key: &str, value: &str) -> Result<(), DBError> {
        write_config_inner(tx, key, value)
    }

    fn write_blocks(&self, blocks: &[BlockRecord]) -> Result<(), DBError> {
        for block in blocks {
            self.rocks_db
                .put(block_hash_key(&block.hash), &block.data)
                .map_err(|e| DBError::Error(format!("Cannot write block: {:?}", e)))?;
        }

        Ok(())
    }

    fn read_blocks(&self, hashes: Vec<String>) -> Result<Vec<BlockRecord>, DBError> {
        let mut result: Vec<BlockRecord> = Vec::new();
        for hash in hashes {
            let block_data = self
                .rocks_db
                .get(block_hash_key(&hash))
                .map_err(|e| DBError::Error(format!("Error reading block: {:?}", e)))?
                .ok_or(DBError::Error("No block with hash found".to_owned()))?;

            result.push(BlockRecord {
                hash,
                data: block_data,
            })
        }

        Ok(result)
    }

    fn write_blocks_str(&self, hash: &str, blocks_str: &str) -> Result<(), DBError> {
        self.rocks_db
            .put(working_dir_key(hash), blocks_str)
            .map_err(|_| DBError::Error("Cannot write working dir blocks".to_owned()))
    }

    fn write_commit(tx: &rusqlite::Transaction, commit: Commit) -> Result<(), DBError> {
        tx.execute(
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
        )
        .map_err(|e| DBError::Error(format!("Cannot insert commit object: {:?}", e)))?;

        Ok(())
    }

    fn read_commit(&self, hash: &str) -> Result<Option<Commit>, DBError> {
        let blocks = self
            .rocks_db
            .get(working_dir_key(hash))
            .map_err(|e| DBError::Error(format!("Cannot read working dir key: {:?}", e)))?
            .map(|bs| String::from_utf8(bs).unwrap())
            .ok_or(DBError::Consistency("No working dir found".to_owned()))?;

        self.sqlite_db.query_row("SELECT hash, prev_commit_hash, branch, message, author, date, header FROM commits WHERE hash = ?1", [hash], |row| Ok(Some(Commit {
            hash: row.get(0).expect("No hash found in row"),
            prev_commit_hash: row.get(1).expect("No prev_commit_hash found in row"),
            branch: row.get(2).expect("No branch found in row"),
            message: row.get(3).expect("No message found in row"),
            author: row.get(4).expect("No author found in row"),
            date: row.get(5).expect("No date found in row"),
            header: row.get(6).expect("No header found in row"),
            blocks,
        }))).map_err(|e| DBError::Error(format!("Cannot read commit: {:?}", e)))
    }

    fn read_all_commits(&self) -> Result<Vec<ShortCommitRecord>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT hash, branch, message FROM commits ORDER BY date DESC")
            .map_err(|e| {
                DBError::Fundamental(format!("Cannot prepare read commits query: {:?}", e))
            })?;

        let mut rows = stmt
            .query([])
            .map_err(|e| DBError::Error(format!("Cannot read commits: {:?}", e)))?;

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
            .map_err(|e| {
                DBError::Fundamental(format!("Cannot prepare read commits query: {:?}", e))
            })?;

        let mut rows = stmt
            .query([brach_name])
            .map_err(|e| DBError::Error(format!("Cannot read commits: {:?}", e)))?;

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
            .map_err(|_| DBError::Error("Cannot read current branch name".to_owned()))
            .and_then(|v| {
                v.map_or(
                    Err(DBError::Error("Cannot read current branch name".to_owned())),
                    Ok,
                )
            })
    }

    fn write_current_branch_name(
        tx: &rusqlite::Transaction,
        brach_name: &str,
    ) -> Result<(), DBError> {
        write_config_inner(tx, &current_branch_name_key(), brach_name)
    }

    fn read_all_branches(&self) -> Result<Vec<String>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT name FROM branches")
            .map_err(|e| DBError::Error(format!("Cannot query branches: {:?}", e)))?;
        let mut rows = stmt
            .query([])
            .map_err(|e| DBError::Error(format!("Cannot query branches: {:?}", e)))?;

        let mut result: Vec<String> = vec![];

        while let Ok(Some(data)) = rows.next() {
            let name = data.get(0).map_err(|e| {
                DBError::Fundamental(format!("Branch name not returned in result set: {:?}", e))
            })?;

            result.push(name);
        }

        Ok(result)
    }

    fn read_branch_tip(&self, branch_name: &str) -> Result<Option<String>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT tip FROM branches WHERE name = ?1")
            .map_err(|e| DBError::Error(format!("Cannot query branch: {:?}", e)))?;

        let mut rows = stmt
            .query([branch_name])
            .map_err(|e| DBError::Error(format!("Cannot query branch: {:?}", e)))?;

        let row = rows.next();

        if let Ok(Some(data)) = row {
            Ok(Some(data.get(0).unwrap()))
        } else if let Ok(None) = row {
            Ok(None)
        } else {
            Err(DBError::Error("Cannot query branch".to_owned()))
        }
    }

    fn write_branch_tip(
        tx: &rusqlite::Transaction,
        brach_name: &str,
        tip: &str,
    ) -> Result<(), DBError> {
        tx.execute(
            "INSERT OR REPLACE INTO branches (name, tip) VALUES (?1, ?2)",
            [&brach_name, &tip],
        )
        .map_err(|e| {
            DBError::Error(format!(
                "Cannot create new branch {:?}: {:?}",
                brach_name, e
            ))
        })
        .map(|_| ())
    }

    fn read_current_latest_commit(&self) -> Result<String, DBError> {
        self.read_config(&current_latest_commit_key())
            .map_err(|_| DBError::Error("Cannot read latest commit hash".to_owned()))
            .and_then(|v| {
                v.map_or(
                    Err(DBError::Error("Cannot read latest commit hash".to_owned())),
                    Ok,
                )
            })
    }

    fn write_current_latest_commit(tx: &rusqlite::Transaction, hash: &str) -> Result<(), DBError> {
        write_config_inner(tx, &current_latest_commit_key(), hash)
            .map_err(|e| DBError::Error(format!("Cannot write latest commit hash: {:?}", e)))
    }

    fn execute_in_transaction<F>(&mut self, f: F) -> Result<(), DBError>
    where
        F: FnOnce(&rusqlite::Transaction) -> Result<(), DBError>,
    {
        let tx = self
            .sqlite_db
            .transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)
            .map_err(|_| DBError::Fundamental("Cannot create transaction".to_owned()))?;

        f(&tx)?;

        tx.commit()
            .map_err(|_| DBError::Fundamental("Cannot commit transaction".to_owned()))
    }
}
