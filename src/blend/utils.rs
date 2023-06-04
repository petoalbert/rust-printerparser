use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Error, Read, Write};

use crate::printer_parser::printerparser::PrinterParserOps;

#[derive(Debug, Copy, Clone)]
pub enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}

pub fn to_left<S, A: Clone, B, P: PrinterParserOps<S, A>>(p: P) -> impl PrinterParserOps<S, Either<A, B>> {
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

pub fn from_file(path: &str) -> Result<Vec<u8>, Error> {
    println!("{}", path);
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data[0..7] != *b"BLENDER" {
        let mut decoder = GzDecoder::new(&data[..]);
        let mut gzip_data = Vec::new();
        decoder.read_to_end(&mut gzip_data)?;

        data = gzip_data;
    }

    Ok(data)
}

pub fn to_file(path: &str, data: Vec<u8>) -> Result<(), Error> {
    let file = File::create(path)?;
    let mut gz = GzEncoder::new(file, Compression::default());
    gz.write_all(&data)?;
    gz.finish()?;
    Ok(())
}
