use crate::printer_parser::combinator::*;
use crate::printer_parser::numbers::{be_u32, be_u64, le_u32, le_u64};
use crate::printer_parser::printerparser::*;

use crate::blend::blend::{Endianness, Header, PointerSize};

use super::blend::SimpleParsedBlock;
use super::utils::{to_left, to_right, Either};

pub struct BlendFileParseState {
    pub pointer_size: PointerSize,
    pub endianness: Endianness,
    pub current_block_size: usize,
}

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
    tuple4(tag(b"BLENDER"), pointer_size(), endianness(), version()).map_result(
        |(_, ps, e, v), state| {
            state.endianness = e;
            state.pointer_size = ps;

            Ok(Header {
                pointer_size: ps,
                endianness: e,
                version: v,
            })
        },
        |Header {
             pointer_size,
             endianness,
             version,
         },
         _| {
            Ok((
                b"BLENDER".to_vec(),
                (*pointer_size),
                (*endianness),
                (*version),
            ))
        },
    )
}

pub fn block_code() -> impl PrinterParserOps<BlendFileParseState, [u8; 4]> {
    bytes(4).map_result(
        |bs, _| {
            TryInto::<[u8; 4]>::try_into(bs).map_err(|_| "Cannot convert block code".to_owned())
        },
        |&bs, _| Ok(bs.as_slice().to_vec()),
    )
}

pub fn u32() -> impl PrinterParserOps<BlendFileParseState, u32> {
    map_state(|s: &mut BlendFileParseState| match s.endianness {
        Endianness::Little => Box::new(le_u32()),
        Endianness::Big => Box::new(be_u32()),
    })
}

pub fn size() -> impl PrinterParserOps<BlendFileParseState, u32> {
    u32().map_result(
        |block_size, state| {
            state.current_block_size = block_size
                .try_into()
                .expect("cannot convert block_size (u32) to usize");
            Ok(block_size)
        },
        |&block_size, _| Ok(block_size),
    )
}

pub fn memory_address() -> impl PrinterParserOps<BlendFileParseState, Either<u32, u64>> {
    map_state(
        |state: &mut BlendFileParseState| match (state.endianness, state.pointer_size) {
            (Endianness::Little, PointerSize::Bits32) => Box::new(to_left(le_u32())),
            (Endianness::Big, PointerSize::Bits32) => Box::new(to_left(be_u32())),
            (Endianness::Little, PointerSize::Bits64) => Box::new(to_right(le_u64())),
            (Endianness::Big, PointerSize::Bits64) => Box::new(to_right(be_u64())),
        },
    )
}

pub fn block_data() -> impl PrinterParserOps<BlendFileParseState, Vec<u8>> {
    map_state(|s: &mut BlendFileParseState| Box::new(bytes(s.current_block_size)))
}

pub fn block() -> impl PrinterParserOps<BlendFileParseState, SimpleParsedBlock> {
    tuple3(block_code(), size(), memory_address())
        .zip_with(tuple3(u32(), u32(), block_data()))
        .map(
            |((code, size, addr), (idx, count, data))| SimpleParsedBlock {
                code,
                size,
                memory_address: addr,
                dna_index: idx,
                count,
                data,
            },
            |SimpleParsedBlock {
                 code,
                 size,
                 memory_address,
                 dna_index,
                 count,
                 data,
             }| {
                (
                    ((*code), (*size), (*memory_address)),
                    ((*dna_index), (*count), data.clone()),
                )
            },
        )
}

pub fn blend() -> impl PrinterParserOps<BlendFileParseState, (Header, Vec<SimpleParsedBlock>)> {
    let body = block()
        .many_till(tag(b"ENDB"))
        // .complete() // TODO: debug why there's data at the end
        .map(|(bs, _)| bs, |bs| (bs.clone(), b"ENDB".to_vec()));

    header().zip_with(body)
}
