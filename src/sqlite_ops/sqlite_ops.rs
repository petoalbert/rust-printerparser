/**
 * TABLES:
 *
 * config:
 *      key - primary key, text
 *      value - text
 *
 * https://stackoverflow.com/questions/19337029/insert-if-not-exists-statement-in-sqlite
 * block_data:
 *      hash - primary key, text
 *      data - blob
 *
 * commit:
 *      hash - primary key, text -- the hash of the whole blender file
 *      prev_commit_hash - text
 *      message - text
 *      author - text -- name, email
 *      date - integer
 *      header - blob
 *      blocks - text -- comma separated list of block hashes
 */

type Connection = ();

pub struct BlockRecord<'a> {
    hash: &'a str,
    data: &'a [u8],
}

pub struct Commit<'a> {
    hash: &'a str,
    prev_commit_hash: &'a str,
    message: &'a str,
    author: &'a str,
    date: u128,
    header: &'a [u8],
    blocks: &'a str,
}

pub fn open_db() -> Result<Connection, &'static str> {
    Result::Err("not implemented")
}

pub fn read_config(conn: Connection, key: &str) -> Result<&str, &str> {
    Result::Err("not implemented")
}

pub fn write_config(conn: Connection, key: String, value: &String) -> Result<(), &str> {
    Result::Err("not implemented")
}

pub fn write_blocks(conn: Connection, blocks: Vec<BlockRecord>) -> Result<(), &str> {
    Result::Err("not implemented")
}

pub fn read_commit(conn: Connection, hash: &str) -> Result<(), &str> {
    Result::Err("not implemented")
}

pub fn write_commit(hash: &str, commit: Commit) -> Result<(), &'static str> {
    Result::Err("not implemented")
}
