use parserprinter::{
    blend::{parsers::blend, utils::from_file},
    printer_parser::printerparser::PrinterParser,
};
use std::env;

use parserprinter::blend::utils::to_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let from_path = args.get(1).expect("No path given").as_str();
    let to_path = args.get(2).expect("No path given").as_str();
    let blend_bytes = from_file(from_path).expect("cannot unpack blend file");

    let (_, (header, bytes)) = blend().read(&blend_bytes, &mut ()).unwrap();
    println!("{:?} - {:?}", header, bytes);

    let write_back = blend()
        .write(&(header, bytes), &mut ())
        .expect("cannot serialize blender file");
    to_file(to_path, write_back).expect("cannot write to file")
}
