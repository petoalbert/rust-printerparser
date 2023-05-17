use std::{collections::LinkedList, marker::PhantomData};

/*
Notes:

Not having a flat_map has some limitations: it would be difficult to parse using some context, e.g.
something like python/yaml where the indentation is increased at every level. We could add a state parameter
to print/parse to overcome that limitation, just like parserz does.

 */

// Helper functions

pub const ANY_CHAR: ConsumeChar = ConsumeChar;

#[allow(dead_code)]
pub fn digit() -> impl PrinterParser<char> {
    ANY_CHAR.filter(|c| c.is_digit(10))
}

#[allow(dead_code)]
pub fn char(c: char) -> impl PrinterParser<()> {
    ANY_CHAR.filter(move |x| x == &c).map(|_| (), move |_| c)
}

pub fn string<'a>(s: &'a str) -> impl PrinterParser<()> + 'a {
    ExpectString(s.as_bytes())
}

pub fn preceded_by<A: Clone, PA: PrinterParser<A>, PU: PrinterParser<()>>(
    before: PU,
    parser: PA,
) -> impl PrinterParser<A> {
    before
        .zip_with(parser)
        .map(|(_, a)| a, |a| ((), (*a).clone()))
}

pub fn followed_by<A: Clone, PA: PrinterParser<A>, PU: PrinterParser<()>>(
    parser: PA,
    after: PU,
) -> impl PrinterParser<A> {
    parser
        .zip_with(after)
        .map(|(a, _)| a, |a| ((*a).clone(), ()))
}

#[allow(dead_code)]
pub fn many1<A: Clone, PA: PrinterParser<A>>(combinator: PA) -> impl PrinterParser<LinkedList<A>> {
    let c2 = combinator.clone();

    combinator.zip_with(c2.repeat().clone()).map_result(
        |(a, mut aa)| {
            aa.push_front(a);
            return Ok(aa);
        },
        |a| {
            a.front()
                .ok_or("At least one element required".to_owned())
                .map(|front| (front.clone(), a.clone().split_off(1)))
        },
    )
}

#[allow(dead_code)]
pub fn take_while<A: Clone, PA: PrinterParser<A>, F: Fn(&A) -> bool + Clone>(
    parser: PA,
    predicate: F,
) -> impl PrinterParser<LinkedList<A>> {
    parser.filter(predicate).repeat()
}

#[allow(dead_code)]
pub fn separated_by<
    A: Clone,
    B: Clone,
    PA: PrinterParser<A>,
    PB: PrinterParser<B>,
    PU: PrinterParser<()>,
>(
    a: PA,
    sep: PU,
    b: PB,
) -> impl PrinterParser<(A, B)> {
    a.zip_with(sep).zip_with(b).map(
        |((a, _), b)| (a, b),
        |(a, b)| (((*a).clone(), ()), (*b).clone()),
    )
}

pub fn surrounded_by<
    A: Clone,
    PA: PrinterParser<A>,
    PU1: PrinterParser<()>,
    PU2: PrinterParser<()>,
>(
    before: PU1,
    parser: PA,
    after: PU2,
) -> impl PrinterParser<A> {
    followed_by(preceded_by(before, parser), after)
}

pub fn separated_list<A: Clone, PA: PrinterParser<A>, PU: PrinterParser<()>>(
    parser: PA,
    sep: PU,
) -> impl PrinterParser<LinkedList<A>> {
    let successors = preceded_by(sep, parser.clone()).repeat();
    parser.zip_with(successors).map_result(
        |(v, mut vs)| {
            vs.push_front(v);
            Ok(vs)
        },
        |a| {
            a.front()
                .ok_or("At least one element required".to_owned())
                .map(|front| (front.clone(), a.clone().split_off(1)))
        },
    )
}

// TODO make this somehow part of the PrinterParser trait using 'where A: IntoIterator<Char>'
pub fn as_string(p: impl PrinterParser<LinkedList<char>>) -> impl PrinterParser<String> {
    p.map(
        |cs| cs.into_iter().collect(),
        |s: &String| s.chars().collect(),
    )
}

// PrinterParser trait

pub trait PrinterParser<A: Clone>: Clone {
    fn write(&self, i: A) -> Result<Vec<u8>, String>;

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], A), String>;

    fn print(&self, i: A) -> Result<String, String> {
        self.write(i).and_then(|bytes| {
            std::str::from_utf8(&bytes)
                .map(|s| s.to_owned())
                .map_err(|e| format!("{}", e))
        })
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, A), String> {
        self.read(i.as_bytes()).and_then(|(rem, a)| {
            std::str::from_utf8(rem)
                .map_err(|e| format!("{}", e))
                .map(|s| (s, a))
        })
    }

    fn filter<F: Fn(&A) -> bool>(self, predicate: F) -> Filter<A, F, Self>
    where
        Self: Sized,
    {
        Filter {
            parser: self,
            predicate: predicate,
            phantom: PhantomData,
        }
    }

    fn map<B: Clone, F: Fn(A) -> B, G: Fn(&B) -> A>(self, f: F, g: G) -> Map<A, B, F, G, Self>
    where
        Self: Sized,
    {
        Map {
            parser: self,
            f: f,
            g: g,
            phantom: PhantomData,
        }
    }

    fn map_result<B: Clone, F: Fn(A) -> Result<B, String>, G: Fn(&B) -> Result<A, String>>(
        self,
        f: F,
        g: G,
    ) -> MapResult<A, B, F, G, Self>
    where
        Self: Sized,
    {
        MapResult {
            parser: self,
            f: f,
            g: g,
            phantom: PhantomData,
        }
    }

    fn zip_with<B: Clone, P: PrinterParser<B>>(self, other: P) -> ZipWith<A, B, Self, P>
    where
        Self: Sized,
    {
        ZipWith {
            a: self,
            b: other,
            phantom: PhantomData,
        }
    }

    fn repeat(self) -> Rep<A, Self>
    where
        Self: Sized,
    {
        Rep {
            parser: self,
            phantom: PhantomData,
        }
    }

    fn or<B: Clone, PB: PrinterParser<B>>(self, other: PB) -> Alt<A, B, Self, PB>
    where
        A: Clone,
    {
        Alt {
            a: self,
            b: other,
            phantom: PhantomData,
        }
    }
}

// Parser structs

#[derive(Clone)]
pub struct Alt<A: Clone, B: Clone, PA: PrinterParser<A>, PB: PrinterParser<B>> {
    a: PA,
    b: PB,
    phantom: PhantomData<(A, B)>,
}

#[derive(Clone)]
pub struct Filter<A: Clone, F: Fn(&A) -> bool, P: PrinterParser<A>> {
    parser: P,
    predicate: F,
    phantom: PhantomData<A>,
}

#[derive(Clone)]
pub struct Map<A: Clone, B: Clone, F: Fn(A) -> B, G: Fn(&B) -> A, P: PrinterParser<A> + Sized> {
    parser: P,
    f: F,
    g: G,
    phantom: PhantomData<(A, B)>,
}

#[derive(Clone)]
pub struct MapResult<
    A: Clone,
    B: Clone,
    F: Fn(A) -> Result<B, String>,
    G: Fn(&B) -> Result<A, String>,
    P: PrinterParser<A> + Sized,
> {
    parser: P,
    f: F,
    g: G,
    phantom: PhantomData<(A, B)>,
}

#[derive(Clone)]
pub struct ConsumeChar;

#[derive(Clone)]
pub struct ExpectString<'a>(&'a [u8]);

#[derive(Clone)]
pub struct ZipWith<A: Clone, B: Clone, PA: PrinterParser<A>, PB: PrinterParser<B>> {
    a: PA,
    b: PB,
    phantom: PhantomData<(A, B)>,
}

#[derive(Clone)]
pub struct Rep<A: Clone, P: PrinterParser<A>> {
    parser: P,
    phantom: PhantomData<A>,
}

#[derive(Debug, Clone)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

// Parser implementations

impl<A: Clone, B: Clone, PA: PrinterParser<A>, PB: PrinterParser<B>> PrinterParser<Either<A, B>>
    for Alt<A, B, PA, PB>
{
    fn write(&self, i: Either<A, B>) -> Result<Vec<u8>, String> {
        match i {
            Either::Left(a) => self.a.write(a),
            Either::Right(b) => self.b.write(b),
        }
    }

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], Either<A, B>), String> {
        match self.a.read(i) {
            Ok((rem, a)) => Ok((rem, Either::Left(a))),
            Err(_) => match self.b.read(i) {
                Ok((rem, b)) => Ok((rem, Either::Right(b))),
                Err(e) => Err(e),
            },
        }
    }
}

impl<'a> PrinterParser<()> for ExpectString<'a> {
    fn write(&self, _: ()) -> Result<Vec<u8>, String> {
        Ok(self.0.to_vec())
    }

    fn read<'b>(&self, i: &'b [u8]) -> Result<(&'b [u8], ()), String> {
        if i.len() < self.0.len() {
            Err(format!("Cannot match {}, not enough input", "FIXME"))
        } else {
            let s = &i[..(self.0.len())];
            let remainder = &i[self.0.len()..];
            if s == self.0 {
                Ok((remainder, ()))
            } else {
                Err(format!("Expected '{}' but received '{}'", "FIXME", "FIXME"))
                // TODO
            }
        }
    }
}

impl<A: Clone, F: Fn(&A) -> bool + Clone, P: PrinterParser<A>> PrinterParser<A>
    for Filter<A, F, P>
{
    fn write(&self, i: A) -> Result<Vec<u8>, String> {
        self.parser.write(i) // TODO
    }

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], A), String> {
        let (rem, a) = self.parser.read(i)?;
        if (self.predicate)(&a) {
            Ok((rem, a))
        } else {
            Err("Filter predicate didn't match".to_owned())
        }
    }
}

impl PrinterParser<char> for ConsumeChar {
    fn write(&self, i: char) -> Result<Vec<u8>, String> {
        Ok(i.to_string().bytes().collect())
    }

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], char), String> {
        std::str::from_utf8(&i[..1])
            .or_else(|_| std::str::from_utf8(&i[..2]))
            .or_else(|_| std::str::from_utf8(&i[..3]))
            .or_else(|_| std::str::from_utf8(&i[..4]))
            .map_err(|e| format!("{}", e))
            .map(|s| (&i[s.as_bytes().len()..], s.chars().nth(0).unwrap()))
    }
}

impl<A: Clone, B: Clone, F: Fn(A) -> B + Clone, G: Fn(&B) -> A + Clone, P: PrinterParser<A>>
    PrinterParser<B> for Map<A, B, F, G, P>
{
    fn write(&self, i: B) -> Result<Vec<u8>, String> {
        let o = (self.g)(&i);
        self.parser.write(o)
    }

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], B), String> {
        let (rem, a) = self.parser.read(i)?;
        Ok((rem, (self.f)(a)))
    }
}

impl<
        A: Clone,
        B: Clone,
        F: Fn(A) -> Result<B, String> + Clone,
        G: Fn(&B) -> Result<A, String> + Clone,
        P: PrinterParser<A>,
    > PrinterParser<B> for MapResult<A, B, F, G, P>
{
    fn write(&self, i: B) -> Result<Vec<u8>, String> {
        let o = (self.g)(&i)?;
        self.parser.write(o)
    }

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], B), String> {
        let (rem, a) = self.parser.read(i)?;
        let mapped = (self.f)(a)?;
        Ok((rem, mapped))
    }
}

impl<A: Clone, B: Clone, PA: PrinterParser<A>, PB: PrinterParser<B>> PrinterParser<(A, B)>
    for ZipWith<A, B, PA, PB>
{
    fn write(&self, i: (A, B)) -> Result<Vec<u8>, String> {
        let (a, b) = i;
        let mut x = (self.a).write(a)?;
        let mut y = (self.b).write(b)?;
        x.append(&mut y); // TODO Vec is not good for performance here
        Ok(x)
    }

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], (A, B)), String> {
        let (rem1, a) = self.a.read(i)?;
        let (rem2, b) = self.b.read(rem1)?;
        Ok((rem2, (a, b)))
    }
}

impl<A: Clone, P: PrinterParser<A>> PrinterParser<LinkedList<A>> for Rep<A, P> {
    fn write(&self, x: LinkedList<A>) -> Result<Vec<u8>, String> {
        x.into_iter()
            .map(|item| (self.parser).write(item))
            .collect::<Result<Vec<Vec<u8>>, String>>()
            .map(|vs| vs.concat()) // TODO bad performance
    }

    fn read<'a>(&self, i: &'a [u8]) -> Result<(&'a [u8], LinkedList<A>), String> {
        let mut elements = LinkedList::new();
        let mut rem = i;
        loop {
            let res = self.parser.read(rem);
            match res {
                Err(_) => break,
                Ok((rem1, a)) => {
                    rem = rem1;
                    elements.push_front(a);
                }
            }
        }
        Ok((rem, elements.into_iter().rev().collect::<LinkedList<A>>()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_expected() {
        assert!(matches!(digit().parse("2"), Ok(("", '2'))));
        assert!(matches!(digit().parse("a"), Err(_)));
        assert_eq!(digit().print('2').unwrap(), "2")
    }

    #[test]
    fn test_string() {
        assert!(matches!(
            string("hello").parse("hello there"),
            Ok((" there", ()))
        ));

        assert_eq!(string("hello").print(()).unwrap(), "hello")
    }

    #[test]
    fn test_preceded_by() {
        let grammar = preceded_by(char('*'), string("hello"));
        assert!(matches!(grammar.parse("*hello"), Ok(("", ()))));
        assert_eq!(string("*hello").print(()).unwrap(), "*hello")
    }

    #[test]
    fn test_repeat() {
        let grammar = string("rust").repeat();

        let values = vec![(), (), ()];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("rustrustrust") {
            Ok(("", result)) => assert_eq!(result, list_for_parse),
            _ => panic!("Unexpected value"),
        }

        let values = vec![(), ()];
        let mut list_for_print = LinkedList::new();

        list_for_print.extend(values);

        assert_eq!(grammar.print(list_for_print).unwrap(), "rustrust")
    }

    #[test]
    fn test_many1() {
        let grammar = many1(string("rust"));
        let values = vec![(), ()];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("rustrust") {
            Ok(("", result)) => assert_eq!(result, list_for_parse),
            _ => panic!("Unexpected value"),
        }

        let values = vec![()];
        let mut list_for_parse2 = LinkedList::new();
        list_for_parse2.extend(values);

        assert!(matches!(grammar.parse(""), Err(_)))
    }

    #[test]
    fn test_or() {
        let grammar = string("rust").or(string("haskell"));

        assert!(matches!(grammar.parse("rust"), Ok(("", Either::Left(())))));
        assert!(matches!(
            grammar.parse("haskell"),
            Ok(("", Either::Right(())))
        ));
        assert!(matches!(grammar.parse("javascript"), Err(_)));
        assert_eq!(grammar.print(Either::Left(())).unwrap(), "rust");
        assert_eq!(grammar.print(Either::Right(())).unwrap(), "haskell");
    }

    #[test]
    fn test_take_while() {
        let grammar = take_while(ANY_CHAR, |a| a.is_digit(10));
        let values = vec!['1', '2', '3'];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("123aaaa") {
            Ok(("aaaa", result)) => assert_eq!(result, list_for_parse),
            v => panic!("Unexpected value {:?}", v),
        }

        match grammar.parse("aaaa") {
            Ok(("aaaa", result)) => assert_eq!(result, LinkedList::new()),
            v => panic!("Unexpected value {:?}", v),
        }
    }
}
