use blend::{parsers::header, utils::from_file};
use printer_parser::printerparser::PrinterParser;
use std::env;

mod blend;
mod printer_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No path given").as_str();
    let blend_bytes = from_file(path).expect("cannot unpack blend file");

    let (_, header) = header().read(&blend_bytes, &mut ()).unwrap();
    println!("{:?}", header);
}
