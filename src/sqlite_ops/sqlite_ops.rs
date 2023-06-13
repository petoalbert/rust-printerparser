type Connection = rusqlite::Connection;

pub struct BlockRecord {
    hash: String,
    data: Vec<u8>,
}

pub struct Commit {
    hash: String,
    prev_commit_hash: String,
    message: String,
    author: String,
    date: u64,
    header: Vec<u8>,
    blocks: String,
}

pub fn open_db(path: &str) -> Result<Connection, &'static str> {
    let conn =
        Connection::open(path).unwrap_or_else(|_| panic!("cannot connect to db at {}", path));

    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT
        );
        
        CREATE TABLE IF NOT EXISTS block_data (
            hash TEXT PRIMARY KEY,
            data BLOB
        );
        
        CREATE TABLE IF NOT EXISTS commit (
            hash TEXT PRIMARY KEY,
            prev_commit_hash TEXT,
            message TEXT,
            author TEXT,
            date INTEGER,
            header BLOB,
            blocks TEXT
        );",
        [],
    )
    .expect("Cannot create DB tables");

    Ok(conn)
}

pub fn read_config(conn: &Connection, key: &str) -> Result<Option<String>, &'static str> {
    let mut stmt = conn
        .prepare("SELECT value FROM config WHERE key = ?1")
        .expect("Cannot create query to read config key");

    let mut rows = stmt
        .query(&[key])
        .expect("Cannot run query to read config key");

    if let Ok(Some(row)) = rows.next() {
        Ok(Some(row.get(0).unwrap()))
    } else {
        Ok(None)
    }
}

pub fn write_config(conn: &Connection, key: &String, value: &String) -> Result<(), &'static str> {
    conn.execute(
        "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
        [key, value],
    )
    .expect("Cannot update config key");
    Ok(())
}

pub fn write_blocks(conn: &Connection, blocks: Vec<BlockRecord>) -> Result<(), &'static str> {
    let mut stmt = conn
        .prepare("INSERT OR REPLACE INTO block_data (hash, data) VALUES (?1, ?2)")
        .expect("Cannot prepare query");

    // FIXME: a batching solution should be used here on the long run
    for block in blocks {
        stmt.execute((&block.hash, &block.data))
            .expect("Cannot insert block");
    }

    Ok(())
}

pub fn read_blocks(
    conn: &Connection,
    hashes: Vec<String>,
) -> Result<Vec<BlockRecord>, &'static str> {
    let mut stmt = conn
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

pub fn read_commit(conn: &Connection, hash: &str) -> Result<Commit, &'static str> {
    let mut stmt = conn
        .prepare("SELECT hash, prev_commit_hash, message, author, date, header, blocks FROM commit WHERE hash = ?1")
        .expect("Cannot create query to read config key");

    let mut rows = stmt.query([hash]).expect("Cannot query commit");
    let row = rows
        .next()
        .expect("No rows returned")
        .expect("No data in row");

    Ok(Commit {
        hash: row.get(0).expect("No hash found in row"),
        prev_commit_hash: row.get(1).expect("No prev_commit_hash found in row"),
        message: row.get(2).expect("No message found in row"),
        author: row.get(3).expect("No author found in row"),
        date: row.get(4).expect("No date found in row"),
        header: row.get(5).expect("No header found in row"),
        blocks: row.get(6).expect("No blocks found in row"),
    })
}

pub fn write_commit(conn: &Connection, commit: Commit) -> Result<(), &'static str> {
    conn.execute(
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
