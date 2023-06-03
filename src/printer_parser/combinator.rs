use crate::printer_parser::printerparser::{DefaultValue, MapResult, PrinterParserOps, ZipWith};
use std::rc::Rc;

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
) -> impl PrinterParserOps<S, Vec<A>> {
    let c2 = combinator.clone();

    combinator.zip_with(c2.repeat()).map_result(
        |(v, mut vs), _| {
            let mut vec = vec![v];
            vec.append(&mut vs);
            Ok(vec)
        },
        |a, _| {
            a.first()
                .ok_or("At least one element required".to_owned())
                .map(|front| (front.clone(), a.clone().split_off(1)))
        },
    )
}

#[allow(dead_code)]
pub fn take_while<S, A: Clone, PA: PrinterParserOps<S, A>, F: Fn(&A) -> bool + Clone + 'static>(
    parser: PA,
    predicate: F,
) -> impl PrinterParserOps<S, Vec<A>> {
    parser.filter(predicate).repeat()
}

pub fn take_till<S, A: Clone, PA: PrinterParserOps<S, A>, F: Fn(&A) -> bool + Clone + 'static>(
    parser: PA,
    predicate: F,
) -> impl PrinterParserOps<S, Vec<A>> {
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
) -> impl PrinterParserOps<S, Vec<A>> {
    let successors = preceded_by(sep, parser.clone()).repeat();
    parser.zip_with(successors).map_result(
        |(v, mut vs), _| {
            let mut vec = vec![v];
            vec.append(&mut vs);
            Ok(vec)
        },
        |a, _| {
            a.first()
                .ok_or("At least one element required".to_owned())
                .map(|front| (front.clone(), a.clone().split_off(1)))
        },
    )
}

pub fn tuple3<
    S,
    A: Clone,
    B: Clone,
    C: Clone,
    PA: PrinterParserOps<S, A>,
    PB: PrinterParserOps<S, B>,
    PC: PrinterParserOps<S, C>,
>(
    first: PA,
    second: PB,
    third: PC,
) -> impl PrinterParserOps<S, (A, B, C)> {
    first.zip_with(second).zip_with(third).map(
        |((f, s), t)| (f, s, t),
        |(f, s, t)| (((*f).clone(), (*s).clone()), (*t).clone()),
    )
}

pub fn tuple4<
    S,
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    PA: PrinterParserOps<S, A>,
    PB: PrinterParserOps<S, B>,
    PC: PrinterParserOps<S, C>,
    PD: PrinterParserOps<S, D>,
>(
    first: PA,
    second: PB,
    third: PC,
    fourth: PD,
) -> impl PrinterParserOps<S, (A, B, C, D)> {
    tuple3(first, second, third).zip_with(fourth).map(
        |((f, s, t), fo)| (f, s, t, fo),
        |(f, s, t, fo)| (((*f).clone(), (*s).clone(), (*t).clone()), (*fo).clone()),
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
        match grammar.print(&"hello".to_owned(), &mut ()) {
            Ok(res) => assert_eq!(res, "*hello"),
            Err(e) => panic!("Expected `*hello`, found {:?}", e),
        }
    }

    #[test]
    fn test_repeat1() {
        let grammar = repeat1(consume_char.count(4)).map(
            |p| -> Vec<String> { p.into_iter().map(|w| w.into_iter().collect()).collect() },
            |p| -> Vec<Vec<char>> { p.iter().map(|w| w.chars().collect()).collect() },
        );
        let values = vec!["rust".to_owned(), "lisp".to_owned(), "hack".to_owned()];

        match grammar.parse("rustlisphack", &mut ()) {
            Ok(("", result)) => assert_eq!(result, values),
            _ => panic!("Unexpected value"),
        }

        assert!(matches!(grammar.parse("", &mut ()), Err(_)))
    }

    #[test]
    fn test_take_while() {
        let grammar = take_while(consume_char, |a| a.is_ascii_digit());
        let values = vec!['1', '2', '3'];

        match grammar.parse("123aaaa", &mut ()) {
            Ok(("aaaa", result)) => assert_eq!(result, values),
            v => panic!("Unexpected value {:?}", v),
        }

        match grammar.parse("aaaa", &mut ()) {
            Ok(("aaaa", result)) => assert_eq!(result, vec![]),
            v => panic!("Unexpected value {:?}", v),
        }
    }

    #[test]
    fn test_take_till() {
        let grammar = take_till(consume_char, |a| a.is_ascii_punctuation());
        let (rest, result) = grammar.parse("shut! up", &mut ()).unwrap();
        let values = vec!['s', 'h', 'u', 't'];
        assert_eq!(rest, "! up");
        assert_eq!(result, values);

        let printed = grammar.print(&values, &mut ()).unwrap();
        assert_eq!(printed, "shut")
    }

    #[test]
    fn test_tuple_3() {
        let grammar = tuple3(char('a'), char('b'), char('c'));
        let (rest, result) = grammar.parse("abc", &mut ()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, ('a', 'b', 'c'));

        let result_bad = grammar.parse("abd", &mut ());
        assert!(matches!(result_bad, Err(_)));

        let printed = grammar.print(&result, &mut ()).unwrap();
        assert_eq!(printed, "abc")
    }

    #[test]
    fn test_tuple_4() {
        let grammar = tuple4(char('w'), char('x'), char('y'), char('z'));
        let (rest, result) = grammar.parse("wxyz", &mut ()).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, ('w', 'x', 'y', 'z'));

        let result_bad = grammar.parse("abd", &mut ());
        assert!(matches!(result_bad, Err(_)));

        let printed = grammar.print(&result, &mut ()).unwrap();
        assert_eq!(printed, "wxyz")
    }
}
