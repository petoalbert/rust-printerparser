use flate2::{write::GzEncoder, Compression};
use rayon::prelude::*;

use crate::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, block, header as pheader, BlendFileParseState},
        utils::from_file,
    },
    api::invariants::{
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
        .expect("Cannot read current branch tip")
        .expect("Tip not found for branch");

    let commit_hash = format!("{:x}", blend_hash);

    conn.write_branch_tip(&current_brach_name, &commit_hash)
        .expect("Cannot write commit hash");

    conn.write_current_latest_commit(&commit_hash)
        .expect("Cannot write latest commit");

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

#[cfg(test)]
mod test {
    use tempfile::TempDir;

    use crate::{
        api::test_utils,
        db_ops::{Persistence, DB},
    };

    use super::run_commit_command;

    #[test]
    fn test_initial_commit() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_path);

        run_commit_command("data/untitled.blend", tmp_path, Some("Message".to_owned()));

        let db = Persistence::open(tmp_path).expect("Cannot open test DB");

        // Creates exactly one commit
        assert_eq!(db.read_all_commits().unwrap().len(), 1);

        let commit = db
            .read_commit("a5f92d0a988085ed66c9dcdccc7b9c90")
            .unwrap()
            .unwrap();

        // commit.blocks omitted, too long
        // commit.date omitted, not stable
        // commit.header omitted, not interesting enough
        assert_eq!(commit.author, "Anon");
        assert_eq!(commit.branch, "main");
        assert_eq!(commit.hash, "a5f92d0a988085ed66c9dcdccc7b9c90");
        assert_eq!(commit.message, "Message");
        assert_eq!(commit.prev_commit_hash, "initial");

        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");

        // The current branch name stays the same
        assert_eq!(current_branch_name, "main");

        let latest_commit_hash = db
            .read_current_latest_commit()
            .expect("Cannot read latest commit");

        // The latest commit hash is updated to the hash of the new commit
        assert_eq!(latest_commit_hash, "a5f92d0a988085ed66c9dcdccc7b9c90");

        // The tip of `main` is updated to the hash of the new commit
        let main_tip = db.read_branch_tip("main").unwrap().unwrap();
        assert_eq!(main_tip, "a5f92d0a988085ed66c9dcdccc7b9c90");
    }

    #[test]
    fn test_next_commit() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_path);

        run_commit_command("data/untitled.blend", tmp_path, Some("Message".to_owned()));
        run_commit_command(
            "data/untitled_2.blend",
            tmp_path,
            Some("Message".to_owned()),
        );

        let db = Persistence::open(tmp_path).expect("Cannot open test DB");

        assert_eq!(db.read_all_commits().unwrap().len(), 2);

        let current_branch_name = db
            .read_current_branch_name()
            .expect("Cannot read current branch name");

        // The current branch name stays the same
        assert_eq!(current_branch_name, "main");

        let latest_commit_hash = db
            .read_current_latest_commit()
            .expect("Cannot read latest commit");

        // The latest commit hash is updated to the hash of the new commit
        assert_eq!(latest_commit_hash, "b637ec695e10bed0ce06279d1dc46717");

        // The tip of `main` is updated to the hash of the new commit
        let main_tip = db.read_branch_tip("main").unwrap().unwrap();
        assert_eq!(main_tip, "b637ec695e10bed0ce06279d1dc46717");
    }
}
