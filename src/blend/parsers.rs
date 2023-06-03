use crate::printer_parser::combinator::*;
use crate::printer_parser::primitives::*;
use crate::printer_parser::printerparser::*;

use crate::blend::blend::{Endianness, Header, PointerSize};

type BlendFileParseState = ();

pub fn pointer_size() -> impl PrinterParserOps<BlendFileParseState, PointerSize> {
    byte().map_result(
        |byte, _| match byte {
            b'_' => Ok(PointerSize::Bits32),
            b'-' => Ok(PointerSize::Bits64),
            _ => Err("Invalid pointer size marker".to_owned()),
        },
        |pointer_size, _| match pointer_size {
            PointerSize::Bits32 => Ok(b'_'),
            PointerSize::Bits64 => Ok(b'-'),
        },
    )
}

pub fn endianness() -> impl PrinterParserOps<BlendFileParseState, Endianness> {
    byte().map_result(
        |byte, _| match byte {
            b'v' => Ok(Endianness::Little),
            b'V' => Ok(Endianness::Big),
            _ => Err("Invalid pointer size marker".to_owned()),
        },
        |pointer_size, _| match pointer_size {
            Endianness::Little => Ok(b'v'),
            Endianness::Big => Ok(b'V'),
        },
    )
}

pub fn version() -> impl PrinterParserOps<BlendFileParseState, [u8; 3]> {
    bytes(3).map_result(
        |res, _| match res[..] {
            [a, b, c] => Ok([a, b, c]),
            _ => Err("Cannot parse version triple".to_owned()),
        },
        |version, _| Ok(version.to_vec()),
    )
}

pub fn header() -> impl PrinterParserOps<BlendFileParseState, Header> {
    tuple4(tag(b"BLENDER"), pointer_size(), endianness(), version()).map(
        |(_, ps, e, v)| Header {
            pointer_size: ps,
            endianness: e,
            version: v,
        },
        |Header {
             pointer_size,
             endianness,
             version,
         }| {
            (
                b"BLENDER".to_vec(),
                (*pointer_size),
                (*endianness),
                (*version),
            )
        },
    )
}

pub fn blend() -> impl PrinterParserOps<BlendFileParseState, (Header, Vec<u8>)> {
    let body = byte()
        .many_till(tag(b"ENDB"))
        .complete()
        .map(|(bs, _)| bs, |bs| (bs.clone(), b"ENDB".to_vec()));

    header().zip_with(body)
}
