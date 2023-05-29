use std::rc::Rc;
use std::{collections::LinkedList, marker::PhantomData};

/*
Notes:

Not having a flat_map has some limitations: it would be difficult to parse using some context, e.g.
something like python/yaml where the indentation is increased at every level. We could add a state parameter
to print/parse to overcome that limitation, just like parserz does.

 */

// Helper functions

#[allow(non_upper_case_globals)]
pub const consume_char: ConsumeChar = ConsumeChar;

pub fn map_state<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>> + Clone>(
    f: F,
) -> impl PrinterParserOps<S, A> {
    Rc::new(MapState {
        f: f,
        phantom: PhantomData,
    })
}

struct MapState<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>>> {
    f: F,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>>> PrinterParser<S, A>
    for Rc<MapState<S, A, F>>
{
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        (self.f)(s).write(i, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        (self.f)(s).read(i, s)
    }
}

impl<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>> + Clone> PrinterParserOps<S, A>
    for Rc<MapState<S, A, F>>
{
}

pub fn bytes<S>(count: usize) -> impl PrinterParserOps<S, Vec<u8>> {
    ConsumeBytes(count)
}

pub fn string<'a, S: 'static>(
    s: &'a str,
) -> impl PrinterParserOps<S, String> + DefaultValue<S, String> + 'a {
    ExpectString(s.as_bytes())
        .map_result(
            |a, _| {
                std::str::from_utf8(&a)
                    .map_err(|e| format!("{}", e))
                    .map(|s| s.to_owned())
            },
            |s, _| Ok(s.as_bytes().to_vec()),
        )
        .default(s.to_owned())
}

pub fn tag<'a, S: 'static>(
    bs: &'a [u8],
) -> impl PrinterParserOps<S, Vec<u8>> + DefaultValue<S, Vec<u8>> + 'a {
    ExpectString(bs).default(bs.to_vec())
}

pub trait DefaultValue<S, A> {
    fn value(&self, s: &S) -> Result<A, String>;
}

pub trait PrinterParserOps<S, A>
where
    Self: PrinterParser<S, A> + Clone,
{
    fn filter<F: Fn(&A) -> bool + 'static + Clone>(
        self,
        predicate: F,
    ) -> Rc<MapResult<S, A, A, Self>>
    where
        Self: Sized,
        A: Clone,
    {
        let cloned = predicate.clone();
        self.map_result(
            move |a, _| {
                if cloned(&a) {
                    Ok(a)
                } else {
                    Err("Predicate failed".to_owned())
                }
            },
            move |a, _| {
                if predicate(a) {
                    Ok(a.clone())
                } else {
                    Err("Predicate failed".to_owned())
                }
            },
        )
    }

    fn map<B, F: Fn(A) -> B + 'static, G: Fn(&B) -> A + 'static>(
        self,
        f: F,
        g: G,
    ) -> Rc<MapResult<S, A, B, Self>>
    where
        Self: Sized,
    {
        self.map_result(move |a, _| Ok(f(a)), move |b, _| Ok(g(b)))
    }

    fn map_result<
        B,
        F: Fn(A, &S) -> Result<B, String> + 'static,
        G: Fn(&B, &S) -> Result<A, String> + 'static,
    >(
        self,
        f: F,
        g: G,
    ) -> Rc<MapResult<S, A, B, Self>>
    where
        Self: Sized,
    {
        Rc::new(MapResult {
            parser: self,
            f: Rc::new(f),
            g: Rc::new(g),
            phantom: PhantomData,
        })
    }

    fn zip_with<B, P: PrinterParser<S, B>>(self, other: P) -> Rc<ZipWith<S, A, B, Self, P>>
    where
        Self: Sized,
    {
        Rc::new(ZipWith {
            a: self,
            b: other,
            phantom: PhantomData,
        })
    }

    fn repeat(self) -> Rc<Rep<S, A, Self>>
    where
        Self: Sized,
    {
        Rc::new(Rep {
            parser: self,
            phantom: PhantomData,
        })
    }

    fn count(self, times: usize) -> Rc<Count<S, A, Self>>
    where
        Self: Sized,
    {
        Rc::new(Count {
            times: times,
            parser: self,
            phantom: PhantomData,
        })
    }

    fn or<PB: PrinterParser<S, A>>(self, other: PB) -> Rc<Alt<S, A, Self, PB>> {
        Rc::new(Alt {
            a: self,
            b: other,
            phantom: PhantomData,
        })
    }

    fn as_string(self) -> Rc<MapResult<S, A, String, Self>>
    where
        A: IntoIterator<Item = char> + FromIterator<char>,
    {
        self.map(
            |cs| cs.into_iter().collect(),
            |s: &String| s.chars().collect(),
        )
    }

    fn as_state<F: Fn(A, &mut S) -> Result<(), String>, G: Fn(&S) -> Result<A, String>>(
        self,
        read_state: F,
        write_state: G,
    ) -> Rc<State<S, A, F, G, Self>> {
        Rc::new(State {
            parser: self,
            read_state: read_state,
            write_state: write_state,
            phantom: PhantomData,
        })
    }

    fn default(self, a: A) -> Rc<Default<S, A, Self>> {
        Rc::new(Default {
            parser: self,
            value: a,
            phantom: PhantomData,
        })
    }

    fn as_value<B: Clone + PartialEq + 'static>(self, b: B) -> Rc<MapResult<S, A, B, Self>>
    where
        Self: DefaultValue<S, A> + 'static,
    {
        let cloned = b.clone();
        self.clone().map_result(
            move |_, _| Ok(cloned.clone()), // TODO
            move |v, s| {
                if *v == b {
                    self.value(s)
                } else {
                    Err("Not matching".to_owned())
                }
            },
        )
    }
}

// PrinterParser trait

pub trait PrinterParser<S, A> {
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String>;

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String>;

    fn print(&self, i: A, s: &mut S) -> Result<String, String> {
        self.write(i, s).and_then(|bytes| {
            std::str::from_utf8(&bytes)
                .map(|s| s.to_owned())
                .map_err(|e| format!("{}", e))
        })
    }

    fn parse<'a>(&self, i: &'a str, s: &mut S) -> Result<(&'a str, A), String> {
        self.read(i.as_bytes(), s).and_then(|(rem, a)| {
            std::str::from_utf8(rem)
                .map_err(|e| format!("{}", e))
                .map(|s| (s, a))
        })
    }
}

// Parser structs

pub struct Default<S, A, P: PrinterParser<S, A>> {
    parser: P,
    value: A,
    phantom: PhantomData<S>,
}

pub struct State<
    S,
    A,
    F: Fn(A, &mut S) -> Result<(), String>,
    G: Fn(&S) -> Result<A, String>,
    P: PrinterParser<S, A>,
> {
    parser: P,
    read_state: F,
    write_state: G,
    phantom: PhantomData<(S, A)>,
}

impl<
        S,
        A,
        F: Fn(A, &mut S) -> Result<(), String>,
        G: Fn(&S) -> Result<A, String>,
        P: PrinterParser<S, A>,
    > PrinterParser<S, ()> for Rc<State<S, A, F, G, P>>
{
    fn write(&self, i: (), s: &mut S) -> Result<Vec<u8>, String> {
        let a = (self.write_state)(s)?;
        self.parser.write(a, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], ()), String> {
        let (rem, a) = self.parser.read(i, s)?;
        (self.read_state)(a, s)?;
        Ok((rem, ()))
    }
}

impl<
        S,
        A,
        F: Fn(A, &mut S) -> Result<(), String> + Clone,
        G: Fn(&S) -> Result<A, String> + Clone,
        P: PrinterParser<S, A> + Clone,
    > PrinterParserOps<S, ()> for Rc<State<S, A, F, G, P>>
{
}

impl<
        S,
        A,
        F: Fn(A, &mut S) -> Result<(), String> + Clone,
        G: Fn(&S) -> Result<A, String> + Clone,
        P: PrinterParser<S, A> + Clone,
    > DefaultValue<S, ()> for Rc<State<S, A, F, G, P>>
{
    fn value(&self, s: &S) -> Result<(), String> {
        Ok(())
    }
}

pub struct Defer<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>>> {
    resolve: F,
    phantom: PhantomData<(S, A)>,
}

pub fn defer<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>>>(f: F) -> Rc<Defer<S, A, F>> {
    Rc::new(Defer {
        resolve: f,
        phantom: PhantomData,
    })
}

impl<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>> + Clone> PrinterParserOps<S, A>
    for Rc<Defer<S, A, F>>
{
}

impl<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>> + Clone> PrinterParser<S, A>
    for Rc<Defer<S, A, F>>
{
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        (self.resolve)().write(i, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        (self.resolve)().read(i, s)
    }
}

pub struct Alt<S, A, PA: PrinterParser<S, A>, PB: PrinterParser<S, A>> {
    a: PA,
    b: PB,
    phantom: PhantomData<(A, S)>,
}

impl<S, A, PA: PrinterParser<S, A> + Clone, PB: PrinterParser<S, A> + Clone> Clone
    for Alt<S, A, PA, PB>
{
    fn clone(&self) -> Self {
        Alt {
            a: self.a.clone(),
            b: self.b.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct MapResult<S, A, B, P: PrinterParser<S, A> + Sized> {
    parser: P,
    f: Rc<dyn Fn(A, &S) -> Result<B, String>>,
    g: Rc<dyn Fn(&B, &S) -> Result<A, String>>,
    phantom: PhantomData<(A, B, S)>,
}

#[derive(Clone)]
pub struct ConsumeChar;

#[derive(Clone)]
pub struct ConsumeBytes(usize);

#[derive(Clone)]
pub struct ExpectString<'a>(&'a [u8]);

pub struct ZipWith<S, A, B, PA: PrinterParser<S, A>, PB: PrinterParser<S, B>> {
    a: PA,
    b: PB,
    phantom: PhantomData<(A, B, S)>,
}

impl<S, A, B, PA: PrinterParser<S, A> + Clone, PB: PrinterParser<S, B> + Clone> Clone
    for ZipWith<S, A, B, PA, PB>
{
    fn clone(&self) -> Self {
        ZipWith {
            a: self.a.clone(),
            b: self.b.clone(),
            phantom: PhantomData,
        }
    }
}

impl<
        S,
        A,
        B,
        PA: PrinterParser<S, A> + Clone + DefaultValue<S, A>,
        PB: PrinterParser<S, B> + Clone + DefaultValue<S, B>,
    > DefaultValue<S, (A, B)> for Rc<ZipWith<S, A, B, PA, PB>>
{
    fn value(&self, s: &S) -> Result<(A, B), String> {
        let a = self.a.value(s)?;
        let b = self.b.value(s)?;
        Ok((a, b))
    }
}

pub struct Rep<S, A, P: PrinterParser<S, A>> {
    parser: P,
    phantom: PhantomData<(A, S)>,
}

pub struct Count<S, A, P: PrinterParser<S, A>> {
    times: usize,
    parser: P,
    phantom: PhantomData<(A, S)>,
}

// Parser implementations

impl<S, A, P: PrinterParser<S, A>> PrinterParser<S, A> for Rc<Default<S, A, P>> {
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        self.parser.write(i, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        self.parser.read(i, s)
    }
}

impl<S, A: Clone, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, A> for Rc<Default<S, A, P>> {}

impl<S, A: Clone, P: PrinterParser<S, A>> DefaultValue<S, A> for Rc<Default<S, A, P>> {
    fn value(&self, _: &S) -> Result<A, String> {
        Ok(self.value.clone())
    }
}

impl<S, A: Clone, PA: PrinterParser<S, A>, PB: PrinterParser<S, A>> PrinterParser<S, A>
    for Rc<Alt<S, A, PA, PB>>
{
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        self.a.write(i.clone(), s).or(self.b.write(i, s))
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        self.a.read(i, s).or(self.b.read(i, s))
    }
}

impl<S, A: Clone, PA: PrinterParser<S, A> + Clone, PB: PrinterParser<S, A> + Clone>
    PrinterParserOps<S, A> for Rc<Alt<S, A, PA, PB>>
{
}

impl<'a, S> PrinterParser<S, Vec<u8>> for ExpectString<'a> {
    fn write(&self, v: Vec<u8>, s: &mut S) -> Result<Vec<u8>, String> {
        if v == self.0 {
            Ok(self.0.to_vec())
        } else {
            Err("Not matching".to_owned())
        }
    }

    fn read<'b>(&self, i: &'b [u8], s: &mut S) -> Result<(&'b [u8], Vec<u8>), String> {
        if i.len() < self.0.len() {
            Err(format!("Cannot match {}, not enough input", "FIXME"))
        } else {
            let s = &i[..(self.0.len())];
            let remainder = &i[self.0.len()..];
            if s == self.0 {
                Ok((remainder, s.to_vec()))
            } else {
                Err(format!("Expected '{}' but received '{}'", "FIXME", "FIXME"))
                // TODO
            }
        }
    }
}

impl<'a, S> PrinterParserOps<S, Vec<u8>> for ExpectString<'a> {}

impl<S> PrinterParser<S, Vec<u8>> for ConsumeBytes {
    fn write(&self, i: Vec<u8>, s: &mut S) -> Result<Vec<u8>, String> {
        Ok(i)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], Vec<u8>), String> {
        if i.len() < self.0 {
            Err("input has not enough elements left".to_owned())
        } else {
            Ok((
                &i[(self.0)..],
                i[0..(self.0)].to_owned().into_iter().collect(),
            ))
        }
    }
}

impl<S> PrinterParserOps<S, Vec<u8>> for ConsumeBytes {}

impl<S> PrinterParser<S, char> for ConsumeChar {
    fn write(&self, i: char, s: &mut S) -> Result<Vec<u8>, String> {
        Ok(i.to_string().bytes().collect())
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], char), String> {
        if i.is_empty() {
            return Err("0 length input encountered".to_owned());
        }

        std::str::from_utf8(&i[..1])
            .or_else(|_| std::str::from_utf8(&i[..2]))
            .or_else(|_| std::str::from_utf8(&i[..3]))
            .or_else(|_| std::str::from_utf8(&i[..4]))
            .map_err(|e| format!("{}", e))
            .map(|s| (&i[s.as_bytes().len()..], s.chars().next().unwrap()))
    }
}

impl<S> PrinterParserOps<S, char> for ConsumeChar {}

impl<S, A, B, P: PrinterParser<S, A>> PrinterParser<S, B> for Rc<MapResult<S, A, B, P>> {
    fn write(&self, i: B, s: &mut S) -> Result<Vec<u8>, String> {
        let o = (self.g)(&i, s)?;
        self.parser.write(o, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], B), String> {
        let (rem, a) = self.parser.read(i, s)?;
        let mapped = (self.f)(a, s)?;
        Ok((rem, mapped))
    }
}

impl<S, A, B, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, B> for Rc<MapResult<S, A, B, P>> {}

impl<S, A, B, P: PrinterParser<S, A> + Clone + DefaultValue<S, A>> DefaultValue<S, B>
    for Rc<MapResult<S, A, B, P>>
{
    fn value(&self, s: &S) -> Result<B, String> {
        let a = self.parser.value(s)?;
        (self.f)(a, s)
    }
}

impl<S, A, B, PA: PrinterParser<S, A>, PB: PrinterParser<S, B>> PrinterParser<S, (A, B)>
    for Rc<ZipWith<S, A, B, PA, PB>>
{
    fn write(&self, i: (A, B), s: &mut S) -> Result<Vec<u8>, String> {
        let (a, b) = i;
        let mut x = (self.a).write(a, s)?;
        let mut y = (self.b).write(b, s)?;
        x.append(&mut y); // TODO Vec is not good for performance here
        Ok(x)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], (A, B)), String> {
        let (rem1, a) = self.a.read(i, s)?;
        let (rem2, b) = self.b.read(rem1, s)?;
        Ok((rem2, (a, b)))
    }
}

impl<S, A, B, PA: PrinterParser<S, A> + Clone, PB: PrinterParser<S, B> + Clone>
    PrinterParserOps<S, (A, B)> for Rc<ZipWith<S, A, B, PA, PB>>
{
}

impl<S, A, P: PrinterParser<S, A>> PrinterParser<S, LinkedList<A>> for Rc<Rep<S, A, P>> {
    fn write(&self, x: LinkedList<A>, s: &mut S) -> Result<Vec<u8>, String> {
        x.into_iter()
            .map(|item| (self.parser).write(item, s))
            .collect::<Result<Vec<Vec<u8>>, String>>()
            .map(|vs| vs.concat()) // TODO bad performance
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], LinkedList<A>), String> {
        let mut elements = LinkedList::new();
        let mut rem = i;
        loop {
            let res = self.parser.read(rem, s);
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

impl<S, A, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, LinkedList<A>> for Rc<Rep<S, A, P>> {}

impl<S, A, P: PrinterParser<S, A>> PrinterParser<S, LinkedList<A>> for Rc<Count<S, A, P>> {
    fn write(&self, x: LinkedList<A>, s: &mut S) -> Result<Vec<u8>, String> {
        x.into_iter()
            .map(|item| (self.parser).write(item, s))
            .collect::<Result<Vec<Vec<u8>>, String>>()
            .map(|vs| vs.concat()) // TODO bad performance
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], LinkedList<A>), String> {
        let mut elements = LinkedList::new();
        let mut rem = i;

        for _ in 0..self.times {
            let res = self.parser.read(rem, s);
            match res {
                Err(_) => return Err("count".to_owned()),
                Ok((rem1, a)) => {
                    rem = rem1;
                    elements.push_front(a);
                }
            }
        }

        Ok((rem, elements.into_iter().rev().collect::<LinkedList<A>>()))
    }
}

impl<S, A, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, LinkedList<A>>
    for Rc<Count<S, A, P>>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printer_parser::primitives::digit;

    #[test]
    fn test_digit_expected() {
        assert!(matches!(digit().parse("2", &mut ()), Ok(("", '2'))));
        assert!(matches!(digit().parse("a", &mut ()), Err(_)));
        assert_eq!(digit().print('2', &mut ()).unwrap(), "2")
    }

    #[test]
    fn test_string() {
        let grammar = string("hello");
        let (rest, result) = grammar.parse("hello there", &mut ()).unwrap();
        assert_eq!(rest, " there");
        assert_eq!(result, "hello");
        assert_eq!(grammar.print("hello".to_owned(), &mut ()).unwrap(), "hello");

        assert!(matches!(grammar.parse("general kenobi", &mut ()), Err(_)));
    }

    #[test]
    fn test_tag() {
        let grammar = tag(b"hello");
        let (rest, result) = grammar.read(b"hello there", &mut ()).unwrap();
        assert_eq!(rest, b" there");
        assert_eq!(result, b"hello");

        assert_eq!(
            tag(b"hello").write(b"hello".to_vec(), &mut ()).unwrap(),
            b"hello"
        );

        assert!(matches!(
            tag(b"hello").read(b"general kenobi", &mut ()),
            Err(_)
        ));
    }

    #[test]
    fn test_repeat() {
        let grammar = string("rust").repeat();

        let values = vec!["rust".to_owned(), "rust".to_owned(), "rust".to_owned()];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("rustrustrust", &mut ()) {
            Ok(("", result)) => assert_eq!(result, list_for_parse),
            _ => panic!("Unexpected value"),
        }

        let values = vec!["rust".to_owned(), "rust".to_owned()];
        let mut list_for_print = LinkedList::new();

        list_for_print.extend(values);

        assert_eq!(grammar.print(list_for_print, &mut ()).unwrap(), "rustrust")
    }

    #[test]
    fn test_count_ok() {
        let grammar = digit().count(3);

        let (rest, result) = grammar.parse("123hello", &mut ()).unwrap();
        assert_eq!(rest, "hello");

        let mut list = LinkedList::new();
        list.extend(vec!['1', '2', '3']);
        assert_eq!(result, list);

        let printed = grammar.print(list, &mut ()).unwrap();
        assert_eq!(printed, "123")
    }

    #[test]
    fn test_count_not_enough() {
        let grammar = digit().count(5);

        let result = grammar.parse("123hello", &mut ());
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_count_too_many() {
        let grammar = digit().count(2);

        let (rest, result) = grammar.parse("123hello", &mut ()).unwrap();
        assert_eq!(rest, "3hello");

        let mut list = LinkedList::new();
        list.extend(vec!['1', '2']);
        assert_eq!(result, list)
    }

    #[test]
    fn test_or() {
        let grammar = string("rust").or(string("haskell"));

        let parsed_rust = "rust".to_owned();
        let parsed_haskell = "haskell".to_owned();
        let (rest, result) = grammar.parse("rust", &mut ()).unwrap();
        assert_eq!(result, "rust");
        assert_eq!(rest, "");

        let (rest, result) = grammar.parse("haskell", &mut ()).unwrap();
        assert_eq!(result, "haskell");
        assert_eq!(rest, "");

        assert!(matches!(grammar.parse("javascript", &mut ()), Err(_)));
        assert_eq!(grammar.print(parsed_rust, &mut ()).unwrap(), "rust");
        assert_eq!(grammar.print(parsed_haskell, &mut ()).unwrap(), "haskell");
    }
}
