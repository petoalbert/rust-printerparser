use blend::{parsers::blend, utils::from_file};
use printer_parser::printerparser::PrinterParser;
use std::env;

use crate::blend::utils::to_file;

mod blend;
mod printer_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No path given").as_str();
    let blend_bytes = from_file(path).expect("cannot unpack blend file");

    let (rest, (header, bytes)) = blend().read(&blend_bytes, &mut ()).unwrap();
    assert!(rest.is_empty()); // TODO: this could become its own combinator
    println!("{:?} - {:?}", header, bytes);

    let write_back = blend()
        .write(&(header, bytes), &mut ())
        .expect("cannot serialize blender file");
    to_file(path, write_back).expect("cannot write to file")
}
