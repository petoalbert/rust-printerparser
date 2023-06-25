use flate2::{write::GzEncoder, Compression};
use rayon::prelude::*;

use crate::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, block, header as pheader, BlendFileParseState},
        utils::from_file,
    },
    db_ops::{BlockRecord, Commit, Persistence, DB},
    printer_parser::printerparser::PrinterParser,
};

use std::{
    io::Write,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use super::utils::hash_list;

pub fn run_commit_command(file_path: &str, db_path: &str, message: Option<String>) {
    let start_commit_command = Instant::now();
    println!("Reading {:?}...", file_path);
    let start = Instant::now();
    let blend_bytes = from_file(file_path).expect("cannot unpack blend file");
    let duration_read_file = start.elapsed();

    println!("Took {:?}", duration_read_file);

    let mut parse_state = BlendFileParseState {
        pointer_size: PointerSize::Bits32,
        endianness: Endianness::Little,
        current_block_size: 0,
    };

    println!("Parsing blocks {:?}...", file_path);
    let start_parse = Instant::now();
    let (_, (header, blocks)) = blend().read(&blend_bytes, &mut parse_state).unwrap();
    let duration_parse = start_parse.elapsed();
    println!("Took {:?}", duration_parse);

    println!("Number of blocks: {:?}", blocks.len());

    println!("Hashing blocks {:?}...", file_path);
    let start_hash_blocks = Instant::now();
    let block_data: Vec<BlockRecord> = blocks
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
    let duration_hash_blocks = start_hash_blocks.elapsed();
    println!("Took {:?}", duration_hash_blocks);

    let conn = Persistence::open(db_path).expect("cannot open DB");

    println!("Writing blocks {:?}...", file_path);
    let start_write_blocks = Instant::now();
    conn.write_blocks(&block_data[..])
        .expect("Cannot write blocks");
    let duration_write_blocks = start_write_blocks.elapsed();
    println!("Took {:?}", duration_write_blocks);

    let header_bytes = pheader().write(&header, &mut parse_state).unwrap();
    let block_hashes: Vec<String> = block_data.iter().map(move |b| b.hash.to_owned()).collect();
    let blocks_str = hash_list().print(&block_hashes, &mut ()).unwrap();

    println!("Hashing {:?}...", file_path);
    let start_hash = Instant::now();
    let blend_hash = md5::compute(&blocks_str);
    let duration_hash_file = start_hash.elapsed();
    println!("Took {:?}", duration_hash_file);

    let name = conn
        .read_config("name")
        .expect("Cannot read name")
        .unwrap_or("Anon".to_owned());

    let current_brach_name = conn
        .read_current_branch_name()
        .expect("Cannot read current branch name");

    let tip = conn
        .read_branch_tip(&current_brach_name)
        .expect("Cannot read current branch tip");

    let commit_hash = format!("{:x}", blend_hash);

    conn.write_branch_tip(&commit_hash, &commit_hash)
        .expect("Cannot write commit hash");

    let commit = Commit {
        hash: commit_hash,
        prev_commit_hash: tip,
        branch: current_brach_name,
        message: message.unwrap_or_default(),
        author: name,
        date: timestamp(),
        header: header_bytes,
        blocks: blocks_str,
    };

    conn.write_commit(commit).expect("cannot write commit");
    println!("Committing took {:?}", start_commit_command.elapsed());
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
