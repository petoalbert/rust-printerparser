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
