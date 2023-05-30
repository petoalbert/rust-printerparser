use libflate::gzip::Decoder;
use std::fs::File;
use std::io::{Error, Read};

pub fn from_file(path: &str) -> Result<Vec<u8>, Error> {
    println!("{}", path);
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data[0..7] != *b"BLENDER" {
        let mut decoder = Decoder::new(&data[..])?;
        let mut gzip_data = Vec::new();
        decoder.read_to_end(&mut gzip_data)?;

        data = gzip_data;
    }

    Ok(data)
}
