use crate::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, BlendFileParseState},
        utils::{from_file, to_file_transactional},
    },
    printer_parser::printerparser::PrinterParser,
};

pub fn run_command_test(from_file_path: String, to_file_path: String) {
    let blend_bytes = from_file(&from_file_path).expect("cannot unpack blend file");

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

    to_file_transactional(&to_file_path, write_back).expect("cannot write to file")
}
