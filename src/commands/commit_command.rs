use flate2::{write::GzEncoder, Compression};
use rayon::prelude::*;

use crate::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, block, header as pheader, BlendFileParseState},
        utils::from_file,
    },
    commands::invariants::{
        check_current_branch_current_commit_set, check_no_detached_head_invariant,
    },
    db_ops::{BlockRecord, Commit, Persistence, DB},
    measure_time,
    printer_parser::printerparser::PrinterParser,
};

use std::{
    io::Write,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use super::utils::hash_list;

pub fn run_commit_command(file_path: &str, db_path: &str, message: Option<String>) {
    let conn = Persistence::open(db_path).expect("cannot open DB");

    check_current_branch_current_commit_set(&conn);
    check_no_detached_head_invariant(&conn);

    let start_commit_command = Instant::now();
    let blend_bytes = measure_time!(format!("Reading {:?}", file_path), {
        from_file(file_path).expect("cannot unpack blend file")
    });

    let mut parse_state = BlendFileParseState {
        pointer_size: PointerSize::Bits32,
        endianness: Endianness::Little,
        current_block_size: 0,
    };

    let (_, (header, blocks)) = measure_time!(format!("Parsing blocks {:?}", file_path), {
        blend().read(&blend_bytes, &mut parse_state).unwrap()
    });

    println!("Number of blocks: {:?}", blocks.len());

    let block_data: Vec<BlockRecord> = measure_time!(format!("Hashing blocks {:?}", file_path), {
        blocks
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
            .collect()
    });

    measure_time!(format!("Writing blocks {:?}", file_path), {
        conn.write_blocks(&block_data[..])
            .expect("Cannot write blocks")
    });

    let header_bytes = pheader().write(&header, &mut parse_state).unwrap();
    let block_hashes: Vec<String> = measure_time!("Collecting block hashes", {
        block_data.iter().map(move |b| b.hash.to_owned()).collect()
    });
    let blocks_str = measure_time!("Printing hash list", {
        hash_list().print(&block_hashes, &mut ()).unwrap()
    });

    let blend_hash = measure_time!(format!("Hashing {:?}", file_path), {
        md5::compute(&blocks_str)
    });

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

// #[cfg(test)]
// mod test {
//     use std::fs;

//     #[test]
//     fn test_test() {
//         let paths = fs::read_dir("./").unwrap();

//         for path in paths {
//             println!("Name: {}", path.unwrap().path().display())
//         }
//     }
// }
