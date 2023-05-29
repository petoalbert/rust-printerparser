use std::rc::Rc;

use crate::printer_parser::printerparser::{
    consume_char, ConsumeChar, Default, MapResult, PrinterParserOps,
};

#[allow(dead_code)]
pub fn digit<S>() -> impl PrinterParserOps<S, char> {
    consume_char.filter(|c| c.is_digit(10))
}

#[allow(dead_code)]
pub fn char<S>(c: char) -> Rc<Default<S, char, Rc<MapResult<S, char, char, ConsumeChar>>>> {
    consume_char.filter(move |x| x == &c).default(c)
}
