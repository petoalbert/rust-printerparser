use crate::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, block, header as pheader, BlendFileParseState},
        utils::from_file,
    },
    db_ops::{open_db, write_blocks, write_commit, BlockRecord, Commit},
    printer_parser::{
        combinator::{repeat1, separated_list},
        primitives::char,
        printerparser::{consume_char, PrinterParser, PrinterParserOps},
    },
};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_commit_command(file_path: &str, db_path: &str) {
    let blend_bytes = from_file(file_path).expect("cannot unpack blend file");

    let blend_hash = md5::compute(&blend_bytes);

    let mut parse_state = BlendFileParseState {
        pointer_size: PointerSize::Bits32,
        endianness: Endianness::Little,
        current_block_size: 0,
    };

    let (_, (header, blocks)) = blend().read(&blend_bytes, &mut parse_state).unwrap();

    let block_data = blocks
        .iter()
        .map(|parsed_block| {
            let block_blob = block()
                .write(parsed_block, &mut parse_state)
                .expect("Cannot write block data");
            let hash = md5::compute(&block_blob);
            BlockRecord {
                hash: format!("{:x}", hash),
                data: block_blob,
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
        message: "test message".to_string(),
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

fn hexa() -> impl PrinterParserOps<(), char> {
    consume_char.filter(|c| c.is_ascii_hexdigit())
}

fn hex_string() -> impl PrinterParserOps<(), String> {
    repeat1(hexa()).map(
        |cs| -> String { cs.into_iter().collect() },
        |s| s.chars().collect(),
    )
}

fn hash_list() -> impl PrinterParserOps<(), Vec<String>> {
    separated_list(hex_string(), char(','))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_list() {
        let (rest, result) = hash_list().parse("aaa,bbb,111,dead1337", &mut ()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, vec!["aaa", "bbb", "111", "dead1337"]);

        let vals = vec![
            "567ab".to_string(),
            "4893edda".to_string(),
            "ca849280bcd".to_string(),
        ];
        let printed = hash_list().print(&vals, &mut ()).unwrap();
        assert_eq!(printed, "567ab,4893edda,ca849280bcd")
    }
}
