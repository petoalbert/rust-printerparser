use std::time::Instant;

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

pub struct SqliteDB {
    conn: rusqlite::Connection,
}

impl DB for SqliteDB {
    fn open(path: &str) -> Result<Self, DBError> {
        let conn =
            Connection::open(path).unwrap_or_else(|_| panic!("cannot connect to db at {}", path));

        conn.execute(
            "CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT
        )",
            [],
        )
        .expect("Cannot create config table");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS block_data (
            hash TEXT PRIMARY KEY,
            data BLOB
        )",
            [],
        )
        .expect("Cannot create block_data table");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS commits (
            hash TEXT PRIMARY KEY,
            prev_commit_hash TEXT,
            message TEXT,
            author TEXT,
            date INTEGER,
            header BLOB,
            blocks TEXT
        )",
            [],
        )
        .expect("Cannot create commits table");

        Ok(Self { conn })
    }

    fn read_config(&self, key: &str) -> Result<Option<String>, DBError> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM config WHERE key = ?1")
            .expect("Cannot create query to read config key");

        let mut rows = stmt
            .query([key])
            .expect("Cannot run query to read config key");

        if let Ok(Some(row)) = rows.next() {
            Ok(Some(row.get(0).unwrap()))
        } else {
            Ok(None)
        }
    }

    fn write_config(&self, key: &str, value: &str) -> Result<(), DBError> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
                [key, value],
            )
            .expect("Cannot update config key");
        Ok(())
    }

    fn write_blocks(&self, blocks: &[BlockRecord]) -> Result<(), DBError> {
        let mut stmt = self
            .conn
            .prepare("INSERT OR IGNORE INTO block_data (hash, data) VALUES (?1, ?2)")
            .expect("Cannot prepare query");

        let mut cnt = 0;
        // FIXME: a batching solution should be used here on the long run
        let mut start_parse = Instant::now();
        for block in blocks {
            stmt.execute((&block.hash, &block.data))
                .expect("Cannot insert block");
            cnt += 1;
            if cnt % 100 == 0 {
                println!(
                    "Inserting 100 blocks took {:?} - {:?}/{:?}",
                    start_parse.elapsed(),
                    cnt,
                    blocks.len()
                );
                start_parse = Instant::now()
            }
        }

        Ok(())
    }

    fn read_blocks(&self, hashes: Vec<String>) -> Result<Vec<BlockRecord>, DBError> {
        let mut stmt = self
            .conn
            .prepare("SELECT hash, data FROM block_data WHERE hash = ?1")
            .expect("Cannot prepare statement");

        let mut result = Vec::new();

        // FIXME: a batching solution should be used here on the long run
        for get_hash in hashes {
            let mut rows = stmt.query([get_hash]).expect("Cannot query block");

            if let Some(row) = rows.next().unwrap() {
                result.push(BlockRecord {
                    hash: row.get(0).expect("Cannot read hash value"),
                    data: row.get(1).expect("Cannot read block data"),
                })
            }
        }

        Ok(result)
    }

    fn write_commit(&self, commit: Commit) -> Result<(), DBError> {
        self.conn.execute(
            "INSERT INTO commits (hash, prev_commit_hash, message, author, date, header, blocks) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                commit.hash,
                commit.prev_commit_hash,
                commit.message,
                commit.author,
                commit.date,
                commit.header,
                commit.blocks,
            ),
        ).expect("Cannot insert commit object");

        Ok(())
    }

    fn read_commit(&self, hash: &str) -> Result<Option<Commit>, DBError> {
        let mut stmt = self.conn
        .prepare("SELECT hash, prev_commit_hash, message, author, date, header, blocks FROM commits WHERE hash = ?1")
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
            blocks: row.get(6).expect("No blocks found in row"),
        }))
    }

    fn read_all_commits(&self) -> Result<Vec<ShortCommitRecord>, DBError> {
        let mut stmt = self
            .conn
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
