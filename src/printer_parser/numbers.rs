use crate::printer_parser::printerparser::{bytes, map_state, PrinterParserOps};

// little endian number types

pub fn le_f32<S>() -> impl PrinterParserOps<S, f32> {
    bytes(4).map(
        |b| f32::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_f64<S>() -> impl PrinterParserOps<S, f64> {
    bytes(4).map(
        |b| f64::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i8<S>() -> impl PrinterParserOps<S, i8> {
    bytes(4).map(
        |b| i8::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i16<S>() -> impl PrinterParserOps<S, i16> {
    bytes(4).map(
        |b| i16::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i32<S>() -> impl PrinterParserOps<S, i32> {
    bytes(4).map(
        |b| i32::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i64<S>() -> impl PrinterParserOps<S, i64> {
    bytes(4).map(
        |b| i64::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_u16<S>() -> impl PrinterParserOps<S, u16> {
    bytes(4).map(
        |b| u16::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_u32<S>() -> impl PrinterParserOps<S, u32> {
    bytes(4).map(
        |b| u32::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_u64<S>() -> impl PrinterParserOps<S, u64> {
    bytes(4).map(
        |b| u64::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

// big endian number types

pub fn be_i8<S>() -> impl PrinterParserOps<S, i8> {
    bytes(4).map(
        |b| i8::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_i16<S>() -> impl PrinterParserOps<S, i16> {
    bytes(4).map(
        |b| i16::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_i32<S>() -> impl PrinterParserOps<S, i32> {
    bytes(4).map(
        |b| i32::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_i64<S>() -> impl PrinterParserOps<S, i64> {
    bytes(4).map(
        |b| i64::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_u16<S>() -> impl PrinterParserOps<S, u16> {
    bytes(4).map(
        |b| u16::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_u32<S>() -> impl PrinterParserOps<S, u32> {
    bytes(4).map(
        |b| u32::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_u64<S>() -> impl PrinterParserOps<S, u64> {
    bytes(4).map(
        |b| u64::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_f32<S>() -> impl PrinterParserOps<S, f32> {
    bytes(4).map(
        |b| f32::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_f64<S>() -> impl PrinterParserOps<S, f64> {
    bytes(4).map(
        |b| f64::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

// endianness-independent number types
#[derive(PartialEq, Debug)]
pub enum Endianness {
    BigEndindan,
    LittleEndian,
}

pub fn i8() -> impl PrinterParserOps<Endianness, i8> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i8()),
        Endianness::LittleEndian => Box::new(le_i8()),
    })
}

pub fn i16() -> impl PrinterParserOps<Endianness, i16> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i16()),
        Endianness::LittleEndian => Box::new(le_i16()),
    })
}

pub fn i32() -> impl PrinterParserOps<Endianness, i32> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i32()),
        Endianness::LittleEndian => Box::new(le_i32()),
    })
}

pub fn i64() -> impl PrinterParserOps<Endianness, i64> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i64()),
        Endianness::LittleEndian => Box::new(le_i64()),
    })
}

pub fn u16() -> impl PrinterParserOps<Endianness, u16> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_u16()),
        Endianness::LittleEndian => Box::new(le_u16()),
    })
}

pub fn u32() -> impl PrinterParserOps<Endianness, u32> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_u32()),
        Endianness::LittleEndian => Box::new(le_u32()),
    })
}

pub fn u64() -> impl PrinterParserOps<Endianness, u64> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_u64()),
        Endianness::LittleEndian => Box::new(le_u64()),
    })
}

pub fn f32() -> impl PrinterParserOps<Endianness, f32> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_f32()),
        Endianness::LittleEndian => Box::new(le_f32()),
    })
}

pub fn f64() -> impl PrinterParserOps<Endianness, f64> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_f64()),
        Endianness::LittleEndian => Box::new(le_f64()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printer_parser::printerparser::PrinterParser;

    #[test]
    fn test_i32() {
        let bytes: [u8; 4] = [1, 2, 3, 4]; // 1 + 2*2^8 + 3*2^16 + 4*2^24 4 + 3*2^8 + 2*2^16 + 2^24

        let (_, i_le) = i32().read(&bytes, &mut Endianness::LittleEndian).unwrap();
        let (_, i_be) = i32().read(&bytes, &mut Endianness::BigEndindan).unwrap();
        let bytes_le = i32().write(i_le, &mut Endianness::LittleEndian).unwrap();
        let bytes_be = i32().write(i_be, &mut Endianness::BigEndindan).unwrap();

        assert_eq!(i_le, 67_305_985);
        assert_eq!(i_be, 16_909_060);
        assert_eq!(bytes_le, bytes);
        assert_eq!(bytes_be, bytes);
    }
}
