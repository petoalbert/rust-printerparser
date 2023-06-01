use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Error, Read, Seek, Write};

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
