use std::path::Path;

type Connection = rusqlite::Connection;

pub struct BlockRecord {
    pub hash: String,
    pub data: Vec<u8>,
}

pub struct Commit {
    pub hash: String,
    pub prev_commit_hash: String,
    pub message: String,
    pub author: String,
    pub date: u64,
    pub header: Vec<u8>,
    pub blocks: String,
}

pub struct ShortCommitRecord {
    pub hash: String,
    pub message: String,
}

#[derive(Debug)]
pub struct DBError(String);

pub trait DB: Sized {
    fn open(path: &str) -> Result<Self, DBError>;

    fn read_config(&self, key: &str) -> Result<Option<String>, DBError>;
    fn write_config(&self, key: &str, value: &str) -> Result<(), DBError>;

    fn write_blocks(&self, blocks: &[BlockRecord]) -> Result<(), DBError>;
    fn read_blocks(&self, hashes: Vec<String>) -> Result<Vec<BlockRecord>, DBError>;

    fn write_commit(&self, commit: Commit) -> Result<(), DBError>;
    fn read_commit(&self, hash: &str) -> Result<Option<Commit>, DBError>;

    fn read_all_commits(&self) -> Result<Vec<ShortCommitRecord>, DBError>;
}

pub struct Persistence {
    rocks_db: rocksdb::DB,
    sqlite_db: rusqlite::Connection,
}

#[inline]
fn config_key(key: &str) -> String {
    format!("config-{:?}", key)
}

#[inline]
fn block_hash_key(key: &str) -> String {
    format!("block-hash-{:?}", key)
}

#[inline]
fn working_dir_key(key: &str) -> String {
    format!("working-dir-{:?}", key)
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
                    message TEXT,
                    author TEXT,
                    date INTEGER,
                    header BLOB
                )",
                [],
            )
            .expect("Cannot create commits table");

        Ok(Self {
            rocks_db,
            sqlite_db,
        })
    }

    fn read_config(&self, key: &str) -> Result<Option<String>, DBError> {
        self.rocks_db
            .get(config_key(key))
            .map_err(|_| DBError(format!("Cannot get value for {:?}", key)))
            .map(|res| res.map(|bs| String::from_utf8(bs).unwrap()))
    }

    fn write_config(&self, key: &str, value: &str) -> Result<(), DBError> {
        self.rocks_db
            .put(config_key(key), value)
            .map_err(|_| DBError(format!("Cannot get value for {:?}", key)))
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
            "INSERT INTO commits (hash, prev_commit_hash, message, author, date, header) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                commit.hash,
                commit.prev_commit_hash,
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

        let mut stmt = self.sqlite_db
        .prepare("SELECT hash, prev_commit_hash, message, author, date, header FROM commits WHERE hash = ?1")
        .expect("Cannot create query to read config key");

        let mut rows = stmt.query([hash]).expect("Cannot query commit");
        let row = rows.next().expect("No rows returned").expect("No data"); // TODO: chained

        Ok(Some(Commit {
            hash: row.get(0).expect("No hash found in row"),
            prev_commit_hash: row.get(1).expect("No prev_commit_hash found in row"),
            message: row.get(2).expect("No message found in row"),
            author: row.get(3).expect("No author found in row"),
            date: row.get(4).expect("No date found in row"),
            header: row.get(5).expect("No header found in row"),
            blocks,
        }))
    }

    fn read_all_commits(&self) -> Result<Vec<ShortCommitRecord>, DBError> {
        let mut stmt = self
            .sqlite_db
            .prepare("SELECT hash, message FROM commits ORDER BY date DESC")
            .expect("Cannot prepare read commits query");

        let mut rows = stmt.query([]).expect("cannot read commits");

        let mut result: Vec<ShortCommitRecord> = vec![];
        while let Ok(Some(data)) = rows.next() {
            result.push(ShortCommitRecord {
                hash: data.get(0).expect("cannot get hash"),
                message: data.get(1).expect("cannot read message"),
            })
        }

        Ok(result)
    }
}
