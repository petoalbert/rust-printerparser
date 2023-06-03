use std::rc::Rc;

use crate::printer_parser::printerparser::{
    consume_char, ConsumeChar, Default, MapResult, PrinterParserOps,
};

#[allow(dead_code)]
pub fn digit<S>() -> impl PrinterParserOps<S, char> {
    consume_char.filter(|c| c.is_ascii_digit())
}

#[allow(dead_code)]
pub fn char<S>(c: char) -> Rc<Default<S, char, Rc<MapResult<S, char, char, ConsumeChar>>>> {
    consume_char.filter(move |x| x == &c).default(c)
}

#[cfg(test)]
mod tests {
    use crate::printer_parser::printerparser::PrinterParser;

    use super::*;

    #[test]
    fn test_digit_expected() {
        assert!(matches!(digit().parse("2", &mut ()), Ok(("", '2'))));
        assert!(matches!(digit().parse("a", &mut ()), Err(_)));
        assert_eq!(digit().print(&'2', &mut ()).unwrap(), "2")
    }
}
