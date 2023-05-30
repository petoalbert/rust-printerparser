use std::{fmt::Debug, num::NonZeroU64};

#[derive(Debug, Copy, Clone)]
pub enum PointerSize {
    Bits32,
    Bits64,
}

impl PointerSize {
    /// Returns the pointer size in bytes.
    pub fn bytes_num(self) -> usize {
        match self {
            PointerSize::Bits32 => 4,
            PointerSize::Bits64 => 8,
        }
    }
}

/// Endianness of the machine used to create the .blend file.
#[derive(Debug, Copy, Clone)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug)]
pub struct Dna {
    pub names: Vec<String>,
    pub types: Vec<DnaType>,
    pub structs: Vec<DnaStruct>,
}

#[derive(Debug)]
pub struct DnaType {
    pub name: String,
    pub bytes_len: usize, //size in bytes of the type
}

#[derive(Debug)]
pub struct DnaField {
    pub type_index: usize, //index on Dna::types array
    pub name_index: usize, //index on Dna::names array
}

#[derive(Debug)]
pub struct DnaStruct {
    pub type_index: usize, //index on Dna::types array
    pub fields: Vec<DnaField>,
}

#[derive(Debug)]
pub struct DnaParseContext {
    endianness: Endianness,
    _pointer_size: PointerSize,
}

#[derive(Debug)]
pub struct BlockData {
    /// The entire binary data of the `Block` in the blend file.
    pub data: Vec<u8>,
    /// The data field can contain more than one struct, count tells us how many there is.
    pub count: usize,
}

// Represents all possible block types found in the blend file.
// `Rend`, `Test` and `Global` are ignored by this crate but are still represented here.
#[derive(Debug)]
pub enum Block {
    Rend,
    Test,
    Global {
        memory_address: NonZeroU64,
        dna_index: usize,
        data: BlockData,
    },
    /// A principal (or root) block is defined by having a two digit code and by the fact that its `dna_index` is always
    /// valid. If we have a pointer to a principal block, we can ignore the type of the pointer and use the block type.
    Principal {
        code: [u8; 2],
        memory_address: NonZeroU64,
        dna_index: usize,
        data: BlockData,
    },
    /// Subsidiary blocks are defined by having the code "DATA", which is ommited here. Their `dna_index` is not
    /// always correct and is only used when whichever field points to them has an "invalid" type (like void*).
    Subsidiary {
        memory_address: NonZeroU64,
        dna_index: usize,
        data: BlockData,
    },
    /// The DNA of the blend file. Used to interpret all the other blocks.
    Dna(Dna),
}

#[derive(Debug, Clone)]
pub struct Header {
    /// The size of the pointer on the machine used to save the blend file.
    pub pointer_size: PointerSize,
    /// The endianness on the machine used to save the blend file.
    pub endianness: Endianness,
    /// The version of Blender used to save the blend file.
    pub version: [u8; 3],
}

#[derive(Debug)]
pub struct RawBlend {
    pub header: Header,
    pub blocks: Vec<Block>,
    pub dna: Dna,
}
