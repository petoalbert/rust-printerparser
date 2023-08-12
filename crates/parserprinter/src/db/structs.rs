use crate::printer_parser::{
    combinator::{repeat1, separated_list},
    primitives::char,
    printerparser::{consume_char, PrinterParserOps},
};

#[derive(Clone)]
pub struct BlockRecord {
    pub hash: String,
    pub data: Vec<u8>,
}

#[derive(Clone)]
pub struct Commit {
    pub hash: String,
    pub prev_commit_hash: String,
    pub project_id: String,
    pub branch: String,
    pub message: String,
    pub author: String,
    pub date: u64,
    pub header: Vec<u8>,
    pub blocks: String,
}

fn hexa() -> impl PrinterParserOps<(), char> {
    consume_char.filter(|c| c.is_ascii_hexdigit())
}

fn hex_string() -> impl PrinterParserOps<(), String> {
    repeat1(hexa()).map(
        |cs| -> String { cs.into_iter().collect() },
        |s| s.chars().collect(),
    )
}

pub fn hash_list() -> impl PrinterParserOps<(), Vec<String>> {
    separated_list(hex_string(), char(','))
}

#[cfg(test)]
mod test {
    use crate::printer_parser::printerparser::PrinterParser;

    use super::*;

    #[test]
    fn test_hash_list() {
        let (rest, result) = hash_list().parse("aaa,bbb,111,dead1337", &mut ()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, vec!["aaa", "bbb", "111", "dead1337"]);

        let vals = vec![
            "567ab".to_string(),
            "4893edda".to_string(),
            "ca849280bcd".to_string(),
        ];
        let printed = hash_list().print(&vals, &mut ()).unwrap();
        assert_eq!(printed, "567ab,4893edda,ca849280bcd")
    }
}
