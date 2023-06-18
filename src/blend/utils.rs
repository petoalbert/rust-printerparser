use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Cursor, Error, Read, Write};
use tempfile::NamedTempFile;
use zstd::decode_all;

use crate::printer_parser::printerparser::PrinterParserOps;

#[derive(Debug, Copy, Clone)]
pub enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}

pub fn to_left<S, A: Clone, B, P: PrinterParserOps<S, A>>(
    p: P,
) -> impl PrinterParserOps<S, Either<A, B>> {
    p.map_result(
        |result, _| Ok(Either::Left(result)),
        |either, _| match either {
            Either::Left(value) => Ok(value.clone()),
            _ => Err("Either::Right found".to_owned()),
        },
    )
}

pub fn to_right<S, A, B: Clone, P: PrinterParserOps<S, B>>(
    p: P,
) -> impl PrinterParserOps<S, Either<A, B>> {
    p.map_result(
        |result, _| Ok(Either::Right(result)),
        |either, _| match either {
            Either::Right(value) => Ok(value.clone()),
            _ => Err("Either::Left found".to_owned()),
        },
    )
}

fn decode_gzip(bytes: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut gzip_data = Vec::new();
    decoder.read_to_end(&mut gzip_data)?;

    Ok(gzip_data)
}

fn decode_zstd(bytes: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut reader = Cursor::new(bytes);
    decode_all(&mut reader)
}

pub fn from_file(path: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data[0..7] != *b"BLENDER" {
        let unzipped = decode_gzip(&data)
            .or(decode_zstd(&data))
            .expect("Cannot unzip blend file");

        data = unzipped;
    }

    Ok(data)
}

pub fn to_file_transactional(path: &str, data: Vec<u8>) -> Result<(), Error> {
    let temp_file = NamedTempFile::new()?;

    let mut gz = GzEncoder::new(&temp_file, Compression::default());
    gz.write_all(&data)?;
    gz.flush()?;
    gz.finish()?;

    temp_file.persist(path)?;

    Ok(())
}
