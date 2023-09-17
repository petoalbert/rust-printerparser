use flate2::{write::GzEncoder, Compression};
use rayon::prelude::*;

use crate::{
    blend::{
        blend_file::{Endianness, PointerSize},
        parsers::{blend, block, header as pheader, BlendFileParseState},
        utils::from_file,
    },
    db::{
        db_ops::{DBError, Persistence, DB},
        structs::{hash_list, BlockRecord},
    },
    measure_time,
    printer_parser::printerparser::PrinterParser,
};

use std::io::Write;

pub fn read_latest_commit_hash_on_branch(
    conn: &Persistence,
    branch_name: &str,
) -> Result<String, DBError> {
    conn.read_branch_tip(branch_name)
        .and_then(|tip| tip.ok_or(DBError::Error("Branch tip does not exist".to_owned())))
}

pub struct BlendFileDataForCheckpoint {
    pub hash: String,
    pub header_bytes: Vec<u8>,
    pub blocks: String,
    pub block_data: Vec<BlockRecord>,
}

pub fn blend_file_data_from_file(
    path_to_blend: &str,
) -> Result<BlendFileDataForCheckpoint, String> {
    let blend_bytes = measure_time!(format!("Reading {:?}", path_to_blend), {
        from_file(path_to_blend).map_err(|_| "Cannot unpack blend file".to_owned())
    })?;

    let mut parse_state = BlendFileParseState {
        pointer_size: PointerSize::Bits32,
        endianness: Endianness::Little,
        current_block_size: 0,
    };

    let (_, (header, blocks)) = measure_time!(format!("Parsing blocks {:?}", path_to_blend), {
        blend().read(&blend_bytes, &mut parse_state).unwrap()
    });

    println!("Number of blocks: {:?}", blocks.len());

    let block_records: Vec<BlockRecord> =
        measure_time!(format!("Hashing blocks {:?}", path_to_blend), {
            blocks
                .par_iter()
                .map(|parsed_block| {
                    let mut state = parse_state.clone();
                    let block_blob = block()
                        .write(parsed_block, &mut state)
                        .map_err(|e| format!("Cannot write block: {:?}", e))?;

                    let hash = md5::compute(&block_blob);

                    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                    encoder
                        .write_all(&block_blob)
                        .map_err(|e| format!("Cannot encode: {:?}", e))?;
                    let compressed = encoder
                        .finish()
                        .map_err(|e| format!("Cannot encode: {:?}", e))?;

                    Ok(BlockRecord {
                        hash: format!("{:x}", hash),
                        data: compressed,
                    })
                })
                .collect::<Vec<Result<BlockRecord, String>>>()
                .into_iter()
                .collect::<Result<Vec<BlockRecord>, String>>()
        })?;

    let header_data = pheader().write(&header, &mut parse_state).unwrap();
    let block_hashes: Vec<String> = measure_time!("Collecting block hashes", {
        block_records
            .iter()
            .map(move |b| b.hash.to_owned())
            .collect()
    });
    let blocks_str = measure_time!("Printing hash list", {
        hash_list().print(&block_hashes, &mut ()).unwrap()
    });

    let blend_hash = measure_time!(format!("Hashing {:?}", path_to_blend), {
        md5::compute(&blocks_str)
    });

    Ok(BlendFileDataForCheckpoint {
        hash: format!("{:x}", blend_hash),
        header_bytes: header_data,
        blocks: blocks_str,
        block_data: block_records,
    })
}
