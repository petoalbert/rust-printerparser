use crate::{
    db::db_ops::{DBError, Persistence, DB},
    exchange::structs::Exchange,
};

pub fn import_exchange(db_path: &str, exchange: Exchange) -> Result<(), DBError> {
    let mut db = Persistence::open(db_path)?;
    db.write_blocks(&exchange.blocks)?;
    // TODO: rename branches
    db.execute_in_transaction(|tx| {
        for commit in exchange.commits.into_iter() {
            Persistence::write_commit(tx, commit)?;
        }

        Ok(())
    })
}
