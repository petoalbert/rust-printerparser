use crate::{
    blend::blend_file::Endianness,
    db::structs::{BlockRecord, Commit},
    printer_parser::{
        combinator::tuple4,
        numbers::{be_u64, le_u64},
        printerparser::{byte, map_state},
    },
    printer_parser::{
        numbers::{be_u32, le_u32},
        printerparser::{consume_char, PrinterParser, PrinterParserOps},
    },
};

/*
   TOOD
   - [ ] pp for commit
   - [ ] pp for blockrecord
*/

pub struct Exchange {
    pub commits: Vec<Commit>,
    pub blocks: Vec<BlockRecord>,
}

pub struct ExchangeState {
    endianness: Endianness,
    count: u32,
}

fn u32() -> impl PrinterParserOps<ExchangeState, u32> {
    map_state(|s: &mut ExchangeState| match s.endianness {
        Endianness::Little => Box::new(le_u32()),
        Endianness::Big => Box::new(be_u32()),
    })
}

fn u64() -> impl PrinterParserOps<ExchangeState, u64> {
    map_state(|s: &mut ExchangeState| match s.endianness {
        Endianness::Little => Box::new(le_u64()),
        Endianness::Big => Box::new(be_u64()),
    })
}

fn vec_of<A: Clone + 'static, PA: PrinterParserOps<ExchangeState, A> + 'static>(
    elem: PA,
) -> impl PrinterParserOps<ExchangeState, Vec<A>> {
    let number_of_elements = u32().map_result(
        |count, state: &mut ExchangeState| {
            state.count = count;
            Ok(count)
        },
        |&count, state| {
            state.count = count;
            Ok(count)
        },
    );

    let parse_elements = map_state(move |state: &mut ExchangeState| {
        let count = state.count;
        Box::new(elem.clone().count(count.try_into().unwrap()))
    });

    number_of_elements.zip_with(parse_elements).map(
        |(_, chars)| chars,
        |elems: &Vec<A>| (elems.len().try_into().unwrap(), elems.clone()),
    )
}

fn sized_string() -> impl PrinterParserOps<ExchangeState, String> {
    vec_of(consume_char).map(
        |chars| chars.into_iter().collect(),
        |str: &String| str.chars().collect(),
    )
}

fn commit() -> impl PrinterParserOps<ExchangeState, Commit> {
    tuple4(
        sized_string(),
        sized_string(),
        sized_string(),
        sized_string(),
    )
    .zip_with(tuple4(
        sized_string(),
        u64(),
        vec_of(byte()),
        sized_string(),
    ))
    .map(
        |((hash, prev_commit_hash, branch, message), (author, date, header, blocks))| Commit {
            hash,
            prev_commit_hash,
            branch,
            message,
            author,
            date,
            header,
            blocks,
        },
        |Commit {
             hash,
             prev_commit_hash,
             branch,
             message,
             author,
             date,
             header,
             blocks,
         }| {
            (
                (
                    hash.clone(),
                    prev_commit_hash.clone(),
                    branch.clone(),
                    message.clone(),
                ),
                (author.clone(), (*date), header.clone(), blocks.clone()),
            )
        },
    )
}

fn block_record() -> impl PrinterParserOps<ExchangeState, BlockRecord> {
    sized_string().zip_with(vec_of(byte())).map(
        |(hash, data)| BlockRecord { hash, data },
        |BlockRecord { hash, data }| (hash.clone(), data.clone()),
    )
}

pub fn exchange() -> impl PrinterParser<ExchangeState, Exchange> {
    vec_of(commit()).zip_with(vec_of(block_record())).map(
        |(commits, blocks)| Exchange { commits, blocks },
        |Exchange { commits, blocks }| (commits.clone(), blocks.clone()),
    )
}

pub fn make_exchange_state() -> ExchangeState {
    let endianness = if cfg!(target_endian = "big") {
        Endianness::Big
    } else {
        Endianness::Little
    };

    ExchangeState {
        endianness,
        count: 0,
    }
}
