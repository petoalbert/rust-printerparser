use crate::db::db_ops::{DBError, Persistence, DB};

pub fn get_latest_commit(db_path: &str) -> Result<String, DBError> {
    Persistence::open(db_path).and_then(|db| db.read_current_latest_commit())
}
