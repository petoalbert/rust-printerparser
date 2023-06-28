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

#[cfg(test)]
mod test {
    use tempfile::{NamedTempFile, TempDir};

    use crate::{commands::test_utils, db_ops::{Persistence, DB}};

    use super::run_restore_command;

    #[test]
    fn test_restore() {
        let tmp_dir = TempDir::new().expect("Cannot create temp dir");
        let tmp_db_path = tmp_dir.path().to_str().expect("Cannot get temp dir path");

        test_utils::init_db(tmp_db_path);

        test_utils::commit(tmp_db_path, "Commit", "data/untitled.blend");
        test_utils::commit(tmp_db_path, "Commit 2", "data/untitled_2.blend");

        let tmp_blend_path = NamedTempFile::new().expect("Cannot create temp file");

        run_restore_command(
            tmp_blend_path.path().to_str().unwrap(),
            tmp_db_path,
            "a5f92d0a988085ed66c9dcdccc7b9c90",
        );

        let db = Persistence::open(tmp_db_path).expect("Cannot open test DB");

        // Number of commits stays the same
        assert_eq!(db.read_all_commits().unwrap().len(), 2);

        let current_branch_name = db.read_current_branch_name().expect("Cannot read current branch name");

        // The current branch name stays the same
        assert_eq!(current_branch_name, "main");

        let latest_commit_hash = db
            .read_current_latest_commit()
            .expect("Cannot read latest commit");

        // The latest commit hash is updated to the hash of the restored commit
        assert_eq!(latest_commit_hash, "a5f92d0a988085ed66c9dcdccc7b9c90");

        // The tip of `main` stays the same
        let main_tip = db.read_branch_tip("main").unwrap().unwrap();
        assert_eq!(main_tip, "b637ec695e10bed0ce06279d1dc46717");
    }
}
