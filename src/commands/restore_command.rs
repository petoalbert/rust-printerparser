use std::{io::Write, time::Instant};

use flate2::write::GzDecoder;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    blend::utils::to_file_transactional,
    db_ops::{Persistence, DB},
    measure_time,
    printer_parser::printerparser::PrinterParser,
};

use super::utils::hash_list;

pub fn run_restore_command(file_path: &str, db_path: &str, hash: &str) {
    let end_to_end_timer = Instant::now();

    let conn = Persistence::open(db_path).expect("Cannot open DB");

    let commit = measure_time!(format!("Reading commit {:?}", hash), {
        conn.read_commit(hash)
            .expect("cannot read commit")
            .expect("no such commit found")
    });

    let block_hashes = measure_time!(format!("Reading blocks {:?}", hash), {
        hash_list()
            .parse(&commit.blocks, &mut ())
            .expect("Cannot parse blocks")
            .1
    });

    let mut header = commit.header;

    let block_data: Vec<Vec<u8>> = measure_time!(format!("Decompressing blocks {:?}", hash), {
        conn.read_blocks(block_hashes)
            .expect("Cannot read block hashes")
            .par_iter()
            .map(|record| {
                let mut writer = Vec::new();
                let mut deflater = GzDecoder::new(writer);
                deflater.write_all(&record.data).unwrap();
                writer = deflater.finish().unwrap();
                writer
            })
            .collect()
    });

    measure_time!(format!("Writing file {:?}", hash), {
        for mut data in block_data {
            header.append(data.as_mut());
        }

        header.append(b"ENDB".to_vec().as_mut());

        to_file_transactional(file_path, header).expect("Cannot write to file");
    });

    conn.write_current_branch_name(&commit.branch)
        .expect("Cannot write current branch");

    conn.write_current_latest_commit(&commit.hash)
        .expect("Cannot set latest commit");

    println!("Checkout took: {:?}", end_to_end_timer.elapsed());
}
