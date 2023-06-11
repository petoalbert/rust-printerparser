use clap::Parser;
use parserprinter::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, BlendFileParseState},
        utils::from_file,
    },
    printer_parser::printerparser::PrinterParser,
};

use parserprinter::blend::utils::to_file_transactional;

#[derive(Parser)]
#[command(about = "The blender version manager tool")]
struct Cli {
    /// path of blender file to read
    #[arg(short, long)]
    from_path: String,

    /// path to write to
    #[arg(short, long)]
    to_path: String,
}

fn main() {
    let args = Cli::parse();
    let blend_bytes = from_file(&args.from_path).expect("cannot unpack blend file");

    let mut parse_state = BlendFileParseState {
        pointer_size: PointerSize::Bits32,
        endianness: Endianness::Little,
        current_block_size: 0,
    };

    let (_, (header, blocks)) = blend().read(&blend_bytes, &mut parse_state).unwrap();
    println!("{:?} - {:?}", header, blocks.len());

    let write_back = blend()
        .write(&(header, blocks), &mut parse_state)
        .expect("cannot serialize blender file");

    to_file_transactional(&args.to_path, write_back).expect("cannot write to file")
}
