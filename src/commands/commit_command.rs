use flate2::{write::GzEncoder, Compression};
use rayon::prelude::*;

use crate::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, block, header as pheader, BlendFileParseState},
        utils::from_file,
    },
    db_ops::{open_db, write_blocks, write_commit, BlockRecord, Commit},
    printer_parser::printerparser::PrinterParser,
};

use std::{
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use super::utils::hash_list;

pub fn run_commit_command(file_path: &str, db_path: &str, message: Option<String>) {
    let blend_bytes = from_file(file_path).expect("cannot unpack blend file");

    let blend_hash = md5::compute(&blend_bytes);

    let mut parse_state = BlendFileParseState {
        pointer_size: PointerSize::Bits32,
        endianness: Endianness::Little,
        current_block_size: 0,
    };

    let (_, (header, blocks)) = blend().read(&blend_bytes, &mut parse_state).unwrap();

    let block_data = blocks
        .par_iter()
        .map(|parsed_block| {
            let mut state = parse_state.clone();
            let block_blob = block()
                .write(parsed_block, &mut state)
                .expect("Cannot write block data");

            let hash = md5::compute(&block_blob);

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(&block_blob)
                .expect("Cannot compress block");
            let compressed = encoder.finish().unwrap();

            BlockRecord {
                hash: format!("{:x}", hash),
                data: compressed,
            }
        })
        .collect();

    let conn = open_db(db_path).expect("cannot open DB");

    write_blocks(&conn, &block_data).expect("Cannot write blocks");

    let header_bytes = pheader().write(&header, &mut parse_state).unwrap();
    let block_hashes: Vec<String> = block_data.iter().map(|b| b.hash.clone()).collect();
    let blocks_str = hash_list().print(&block_hashes, &mut ()).unwrap();

    let commit = Commit {
        hash: format!("{:x}", blend_hash),
        prev_commit_hash: "abcd1234".to_string(),
        message: message.unwrap_or_default(),
        author: "Michelangelo".to_string(),
        date: timestamp(),
        header: header_bytes,
        blocks: blocks_str,
    };

    write_commit(&conn, commit).expect("cannot write commit")
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
