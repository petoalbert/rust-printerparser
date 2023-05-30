use crate::printer_parser::combinator::*;
use crate::printer_parser::primitives::*;
use crate::printer_parser::printerparser::*;

use crate::blend::blend::{Endianness, PointerSize, Header};

pub fn pointer_size() -> impl PrinterParserOps<(), PointerSize> {
    bytes(1).map_result(
        |byte, _| match byte.first() {
            Some(b'_') => Ok(PointerSize::Bits32),
            Some(b'-') => Ok(PointerSize::Bits64),
            _ => Err("Invalid pointer size marker".to_owned()),
        },
        |pointer_size, _| match pointer_size {
            PointerSize::Bits32 => Ok(b"_".to_vec()),
            PointerSize::Bits64 => Ok(b"-".to_vec()),
        },
    )
}

pub fn endianness() -> impl PrinterParserOps<(), Endianness> {
    bytes(1).map_result(
        |byte, _| match byte.first() {
            Some(b'v') => Ok(Endianness::Little),
            Some(b'V') => Ok(Endianness::Big),
            _ => Err("Invalid pointer size marker".to_owned()),
        },
        |pointer_size, _| match pointer_size {
            Endianness::Little => Ok(b"v".to_vec()),
            Endianness::Big => Ok(b"V".to_vec()),
        },
    )
}

pub fn version() -> impl PrinterParserOps<(), [u8; 3]> {
    bytes(3).map_result(
        |res, _| match res[..] {
            [a, b, c] => Ok([a, b, c]),
            _ => Err("Cannot parse version triple".to_owned()),
        },
        |version, _| Ok(version.to_vec()),
    )
}