use parserprinter::{
    blend::{
        blend::{Endianness, PointerSize},
        parsers::{blend, BlendFileParseState},
        utils::from_file,
    },
    cli::{parse_args, Commands},
    printer_parser::printerparser::PrinterParser,
    sqlite_ops::sqlite_ops::{open_db, read_config, write_config},
};

use parserprinter::blend::utils::to_file_transactional;

fn run_command_test(from_file_path: String, to_file_path: String) {
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

fn run_set_name_command(value: String) {
    let conn = open_db("./test.sqlite").expect("Cannot open DB");
    write_config(&conn, &"name".to_string(), &value).expect("Couldn't write name")
}

fn run_get_name_command() {
    let conn = open_db("./test.sqlite").expect("Cannot open DB");
    let name = read_config(&conn, "name")
        .expect("Cannot read name")
        .expect("Name is None");
    println!("{}", name)
}

fn main() {
    let args = parse_args();
    match args.command {
        Commands::Test { from_path, to_path } => run_command_test(from_path, to_path),
        Commands::SetName { value } => run_set_name_command(value),
        Commands::GetName => run_get_name_command(),
    }
}
