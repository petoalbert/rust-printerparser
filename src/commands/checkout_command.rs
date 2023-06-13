use crate::{
    blend::utils::to_file_transactional,
    db_ops::{open_db, read_blocks, read_commit},
    printer_parser::printerparser::PrinterParser,
};

use super::utils::hash_list;

pub fn run_checkout_command(file_path: &str, db_path: &str, hash: &str) {
    let conn = open_db(db_path).expect("Cannot open DB");
    let commit = read_commit(&conn, hash).expect("cannot read commit");
    let block_hashes = hash_list()
        .parse(&commit.blocks, &mut ())
        .expect("Cannot parse blocks")
        .1;
    let mut header = commit.header;

    let block_data = read_blocks(&conn, block_hashes).expect("Cannot read block hashes");

    for mut data in block_data {
        header.append(data.data.as_mut());
    }

    header.append(b"ENDB".to_vec().as_mut());

    to_file_transactional(file_path, header).expect("Cannot write to file")
}
