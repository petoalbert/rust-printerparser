use std::{collections::LinkedList, marker::PhantomData};

/*
Notes:

The struct hierarchy currently owns all parsers. This gets complicated when a parser would be reused multiple
times. What should we do? Implement clone and copy for everything? Or use borrowed values everywhere? Currently
I just worked this around by using a function that creates a parser for separated_list

Could add alternative as another essential combinator.

Not having a flat_map has some limitations: it would be difficult to parse using some context, e.g.
something like python/yaml where the indentation is increased at every level. We could add a state parameter
to print/parse to overcome that limitation, just like parserz does.

 */

// Helper functions

pub const any_char: ConsumeChar = ConsumeChar;

pub fn digit() -> impl PrinterParser<char> {
    any_char.filter(|c| c.is_digit(10))
}

pub fn char(c: char) -> impl PrinterParser<()> {
    any_char.filter(move |x| x == &c).map(|_| (), move |_| c)
}

pub fn string<'a>(s: &'a str) -> impl PrinterParser<()> + 'a {
    ExpectString(s)
}

pub fn preceded_by<A, PA: PrinterParser<A>, PU: PrinterParser<()>>(
    before: PU,
    parser: PA,
) -> impl PrinterParser<A> {
    before.zip_with(parser).map(|(_, a)| a, |a| ((), a))
}

pub fn followed_by<A, PA: PrinterParser<A>, PU: PrinterParser<()>>(
    parser: PA,
    after: PU,
) -> impl PrinterParser<A> {
    parser.zip_with(after).map(|(a, _)| a, |a| (a, ()))
}

pub fn separated_by<A, B, PA: PrinterParser<A>, PB: PrinterParser<B>, PU: PrinterParser<()>>(
    a: PA,
    sep: PU,
    b: PB,
) -> impl PrinterParser<(A, B)> {
    a.zip_with(sep)
        .zip_with(b)
        .map(|((a, _), b)| (a, b), |(a, b)| ((a, ()), b))
}

pub fn surrounded_by<A, PA: PrinterParser<A>, PU1: PrinterParser<()>, PU2: PrinterParser<()>>(
    before: PU1,
    parser: PA,
    after: PU2
) -> impl PrinterParser<A> {
    followed_by(preceded_by(before, parser), after)
}

// TODO this has very bad performance and weird cloning of params, should fix this
pub fn separated_list<A: Clone, PA: PrinterParser<A>, PU: PrinterParser<()>, F: Fn() -> PA>(
    parser: F,
    sep: PU,
) -> impl PrinterParser<LinkedList<A>> {
    let successors = preceded_by(sep, parser()).repeat();
    parser().zip_with(successors).map(
        |(v, mut vs)| {
            vs.push_front(v);
            vs
        },
        |a| {
            // TODO currently we require to have at least one element. Should fix it to allow 0
            (a.front().unwrap().clone(), a.clone().split_off(1))
        },
    )
}

// TODO make this somehow part of the PrinterParser trait using 'where A: IntoIterator<Char>'
pub fn as_string(p: impl PrinterParser<LinkedList<char>>) -> impl PrinterParser<String> {
    p.map(
        |cs| cs.into_iter().collect(),
        |s: String| s.chars().collect(),
    )
}

// PrinterParser trait

pub trait PrinterParser<A> {
    fn print(&self, i: A) -> String;

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, A), String>;

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

    fn map<B, F: Fn(A) -> B, G: Fn(B) -> A>(self, f: F, g: G) -> Map<A, B, F, G, Self>
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

    fn zip_with<B, P: PrinterParser<B>>(self, other: P) -> zip_with<A, B, Self, P>
    where
        Self: Sized,
    {
        zip_with {
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
}

// Parser structs

pub struct Filter<A, F: Fn(&A) -> bool, P: PrinterParser<A>> {
    parser: P,
    predicate: F,
    phantom: PhantomData<A>,
}

pub struct Map<A, B, F: Fn(A) -> B, G: Fn(B) -> A, P: PrinterParser<A> + Sized> {
    parser: P,
    f: F,
    g: G,
    phantom: PhantomData<(A, B)>,
}

pub struct ConsumeChar;

pub struct ExpectString<'a>(&'a str);

pub struct zip_with<A, B, PA: PrinterParser<A>, PB: PrinterParser<B>> {
    a: PA,
    b: PB,
    phantom: PhantomData<(A, B)>,
}

pub struct Rep<A, P: PrinterParser<A>> {
    parser: P,
    phantom: PhantomData<A>,
}

// Parser implementations

impl<'a> PrinterParser<()> for ExpectString<'a> {
    fn print(&self, i: ()) -> String {
        self.0.to_owned()
    }

    fn parse<'b>(&self, i: &'b str) -> Result<(&'b str, ()), String> {
        if i.len() < self.0.len() {
            Err(format!("Cannot match {}, not enough input", self.0))
        } else {
            let s = &i[..(self.0.len())];
            let remainder = &i[self.0.len()..];
            if s == self.0 {
                Ok((remainder, ()))
            } else {
                Err(format!("Expected '{}' but received '{}'", self.0, s))
            }
        }
    }
}

impl<A, F: Fn(&A) -> bool, P: PrinterParser<A>> PrinterParser<A> for Filter<A, F, P> {
    fn print(&self, i: A) -> String {
        self.parser.print(i) // TODO
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, A), String> {
        let (rem, a) = self.parser.parse(i)?;
        if (self.predicate)(&a) {
            Ok((rem, a))
        } else {
            Err("Filter predicate didn't match".to_owned())
        }
    }
}

impl PrinterParser<char> for ConsumeChar {
    fn print(&self, i: char) -> String {
        i.to_string()
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, char), String> {
        let mut chars = i.chars();
        let c = chars.nth(0).ok_or("No more characters".to_owned())?;
        Ok((&i[1..(i.len())], c))
    }
}

impl<A, B, F: Fn(A) -> B, G: Fn(B) -> A, P: PrinterParser<A>> PrinterParser<B>
    for Map<A, B, F, G, P>
{
    fn print(&self, i: B) -> String {
        let o = (self.g)(i);
        self.parser.print(o)
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, B), String> {
        let (rem, a) = self.parser.parse(i)?;
        Ok((rem, (self.f)(a)))
    }
}

impl<A, B, X: PrinterParser<A>, Y: PrinterParser<B>> PrinterParser<(A, B)>
    for zip_with<A, B, X, Y>
{
    fn print(&self, i: (A, B)) -> String {
        let (a, b) = i;
        let x = (self.a).print(a);
        let y = (self.b).print(b);
        x + &y
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, (A, B)), String> {
        let (rem1, a) = self.a.parse(i)?;
        let (rem2, b) = self.b.parse(rem1)?;
        Ok((rem2, (a, b)))
    }
}

impl<A, P: PrinterParser<A>> PrinterParser<LinkedList<A>> for Rep<A, P> {
    fn print(&self, x: LinkedList<A>) -> String {
        x.into_iter()
            .map(|item| (self.parser).print(item))
            .collect::<Vec<String>>()
            .join("")
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, LinkedList<A>), String> {
        let mut elements = LinkedList::new();
        let mut rem = i;
        loop {
            let res = self.parser.parse(rem);
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
