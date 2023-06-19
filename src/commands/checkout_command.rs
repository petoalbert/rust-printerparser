use std::{io::Write, time::Instant};

use flate2::write::GzDecoder;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    blend::utils::to_file_transactional,
    db_ops::{Persistence, DB},
    printer_parser::printerparser::PrinterParser,
};

use super::utils::hash_list;

pub fn run_checkout_command(file_path: &str, db_path: &str, hash: &str) {
    let start = Instant::now();

    println!("Reading commit {:?}", hash);
    let read_checkpoint = Instant::now();
    let conn = Persistence::open(db_path).expect("Cannot open DB");
    let commit = conn
        .read_commit(hash)
        .expect("cannot read commit")
        .expect("no such commit found");
    println!("Reading commit took: {:?}", read_checkpoint.elapsed());

    println!("Reading blocks {:?}...", hash);
    let blocks_checkpoint = Instant::now();
    let block_hashes = hash_list()
        .parse(&commit.blocks, &mut ())
        .expect("Cannot parse blocks")
        .1;

    println!("Reading blocks took: {:?}", blocks_checkpoint.elapsed());

    let mut header = commit.header;

    println!("Decompressing blocks {:?}...", hash);
    let blocks_checkpoint = Instant::now();
    let block_data: Vec<Vec<u8>> = conn
        .read_blocks(block_hashes)
        .expect("Cannot read block hashes")
        .par_iter()
        .map(|record| {
            let mut writer = Vec::new();
            let mut deflater = GzDecoder::new(writer);
            deflater.write_all(&record.data).unwrap();
            writer = deflater.finish().unwrap();
            writer
        })
        .collect();

    println!(
        "Decompressing blocks took: {:?}",
        blocks_checkpoint.elapsed()
    );

    println!("Writing file {:?}...", hash);
    let blocks_checkpoint = Instant::now();
    for mut data in block_data {
        header.append(data.as_mut());
    }

    header.append(b"ENDB".to_vec().as_mut());

    to_file_transactional(file_path, header).expect("Cannot write to file");
    println!("Writing file took: {:?}", blocks_checkpoint.elapsed());
    println!("Checkout took: {:?}", start.elapsed())
}
