use crate::printer_parser::printerparser::{DefaultValue, MapResult, PrinterParserOps, ZipWith};
use std::{collections::LinkedList, rc::Rc};

pub fn preceded_by<
    S,
    A: Clone,
    B,
    PA: PrinterParserOps<S, A>,
    PB: PrinterParserOps<S, B> + DefaultValue<S, B> + 'static,
>(
    before: PB,
    parser: PA,
) -> Rc<MapResult<S, (B, A), A, Rc<ZipWith<S, B, A, PB, PA>>>> {
    before.clone().zip_with(parser).map_result(
        |(_, a), _state| Ok(a),
        move |a, s| before.value(s).map(|b| (b, (*a).clone())),
    )
}

pub fn followed_by<
    S,
    A: Clone,
    B,
    PA: PrinterParserOps<S, A>,
    PB: PrinterParserOps<S, B> + DefaultValue<S, B> + 'static,
>(
    parser: PA,
    after: PB,
) -> Rc<MapResult<S, (A, B), A, Rc<ZipWith<S, A, B, PA, PB>>>> {
    parser.zip_with(after.clone()).map_result(
        |(a, _), _| Ok(a),
        move |a, s| after.value(s).map(|b| ((*a).clone(), b)),
    )
}

#[allow(dead_code)]
pub fn repeat1<S, A: Clone, PA: PrinterParserOps<S, A>>(
    combinator: PA,
) -> impl PrinterParserOps<S, LinkedList<A>> {
    let c2 = combinator.clone();

    combinator.zip_with(c2.repeat()).map_result(
        |(a, mut aa), _| {
            aa.push_front(a);
            Ok(aa)
        },
        |a, _| {
            a.front()
                .ok_or("At least one element required".to_owned())
                .map(|front| (front.clone(), a.clone().split_off(1)))
        },
    )
}

#[allow(dead_code)]
pub fn take_while<S, A: Clone, PA: PrinterParserOps<S, A>, F: Fn(&A) -> bool + Clone + 'static>(
    parser: PA,
    predicate: F,
) -> impl PrinterParserOps<S, LinkedList<A>> {
    parser.filter(predicate).repeat()
}

pub fn take_till<S, A: Clone, PA: PrinterParserOps<S, A>, F: Fn(&A) -> bool + Clone + 'static>(
    parser: PA,
    predicate: F,
) -> impl PrinterParserOps<S, LinkedList<A>> {
    parser.filter(move |a| !predicate(a)).repeat()
}

#[allow(dead_code)]
pub fn separated_by<
    S,
    A: Clone,
    B: Clone,
    PA: PrinterParserOps<S, A>,
    PB: PrinterParserOps<S, B>,
    PU: PrinterParserOps<S, ()>,
>(
    a: PA,
    sep: PU,
    b: PB,
) -> impl PrinterParserOps<S, (A, B)> {
    a.zip_with(sep).zip_with(b).map(
        |((a, _), b)| (a, b),
        |(a, b)| (((*a).clone(), ()), (*b).clone()),
    )
}

pub fn surrounded_by<
    S,
    A: Clone,
    B,
    C,
    PA: PrinterParserOps<S, A>,
    P1: PrinterParserOps<S, B> + DefaultValue<S, B> + 'static,
    P2: PrinterParserOps<S, C> + DefaultValue<S, C> + 'static,
>(
    before: P1,
    parser: PA,
    after: P2,
) -> Rc<
    MapResult<
        S,
        (A, C),
        A,
        Rc<ZipWith<S, A, C, Rc<MapResult<S, (B, A), A, Rc<ZipWith<S, B, A, P1, PA>>>>, P2>>,
    >,
> {
    followed_by(preceded_by(before, parser), after)
}

pub fn separated_list<
    S,
    A: Clone,
    B,
    PA: PrinterParserOps<S, A>,
    PB: PrinterParserOps<S, B> + DefaultValue<S, B> + 'static,
>(
    parser: PA,
    sep: PB,
) -> impl PrinterParserOps<S, LinkedList<A>> {
    let successors = preceded_by(sep, parser.clone()).repeat();
    parser.zip_with(successors).map_result(
        |(v, mut vs), _| {
            vs.push_front(v);
            Ok(vs)
        },
        |a, _| {
            a.front()
                .ok_or("At least one element required".to_owned())
                .map(|front| (front.clone(), a.clone().split_off(1)))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printer_parser::primitives::char;
    use crate::printer_parser::printerparser::{consume_char, string, PrinterParser};

    #[test]
    fn test_preceded_by() {
        let grammar = preceded_by(char('*'), string("hello"));
        let (rest, result) = grammar.parse("*hello", &mut ()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, "hello"); // the '*' is discarded
        match grammar.print("hello".to_owned(), &mut ()) {
            Ok(res) => assert_eq!(res, "*hello"),
            Err(e) => panic!("Expected `*hello`, found {:?}", e),
        }
    }

    #[test]
    fn test_repeat1() {
        let grammar = repeat1(string("rust"));
        let values = vec!["rust".to_owned(), "rust".to_owned()];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("rustrust", &mut ()) {
            Ok(("", result)) => assert_eq!(result, list_for_parse),
            _ => panic!("Unexpected value"),
        }

        let values = vec![()];
        let mut list_for_parse2 = LinkedList::new();
        list_for_parse2.extend(values);

        assert!(matches!(grammar.parse("", &mut ()), Err(_)))
    }

    #[test]
    fn test_take_while() {
        let grammar = take_while(consume_char, |a| a.is_digit(10));
        let values = vec!['1', '2', '3'];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("123aaaa", &mut ()) {
            Ok(("aaaa", result)) => assert_eq!(result, list_for_parse),
            v => panic!("Unexpected value {:?}", v),
        }

        match grammar.parse("aaaa", &mut ()) {
            Ok(("aaaa", result)) => assert_eq!(result, LinkedList::new()),
            v => panic!("Unexpected value {:?}", v),
        }
    }

    #[test]
    fn test_take_till() {
        let grammar = take_till(consume_char, |a| a.is_ascii_punctuation());
        let (rest, result) = grammar.parse("shut! up", &mut ()).unwrap();
        let mut list = LinkedList::new();
        list.extend(vec!['s', 'h', 'u', 't']);
        assert_eq!(rest, "! up");
        assert_eq!(result, list);

        let printed = grammar.print(list, &mut ()).unwrap();
        assert_eq!(printed, "shut")
    }
}
