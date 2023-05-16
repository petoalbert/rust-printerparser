use std::{collections::LinkedList, marker::PhantomData};

/*
Notes:

Could add alternative as another essential combinator.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_expected() {
        assert!(matches!(digit().parse("2"), Ok(("", '2'))))
    }

    #[test]
    fn test_digit_unexpected() {
        assert!(matches!(digit().parse("a"), Err(_)))
    }
}

#[allow(dead_code)]
pub fn char(c: char) -> impl PrinterParser<()> {
    ANY_CHAR.filter(move |x| x == &c).map(|_| (), move |_| c)
}

pub fn string<'a>(s: &'a str) -> impl PrinterParser<()> + 'a {
    ExpectString(s)
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
    fn print(&self, i: A) -> Result<String, String>;

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
pub struct ExpectString<'a>(&'a str);

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
    fn print(&self, i: Either<A, B>) -> Result<String, String> {
        match i {
            Either::Left(a) => self.a.print(a),
            Either::Right(b) => self.b.print(b),
        }
    }

    fn parse<'b>(&self, i: &'b str) -> Result<(&'b str, Either<A, B>), String> {
        match self.a.parse(i) {
            Ok((rem, a)) => Ok((rem, Either::Left(a))),
            Err(_) => match self.b.parse(i) {
                Ok((rem, b)) => Ok((rem, Either::Right(b))),
                Err(e) => Err(e),
            },
        }
    }
}

impl<'a> PrinterParser<()> for ExpectString<'a> {
    fn print(&self, _: ()) -> Result<String, String> {
        Ok(self.0.to_owned())
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

impl<A: Clone, F: Fn(&A) -> bool + Clone, P: PrinterParser<A>> PrinterParser<A>
    for Filter<A, F, P>
{
    fn print(&self, i: A) -> Result<String, String> {
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
    fn print(&self, i: char) -> Result<String, String> {
        Ok(i.to_string())
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, char), String> {
        let mut chars = i.chars();
        let c = chars.nth(0).ok_or("No more characters".to_owned())?;
        Ok((&i[1..(i.len())], c))
    }
}

impl<A: Clone, B: Clone, F: Fn(A) -> B + Clone, G: Fn(&B) -> A + Clone, P: PrinterParser<A>>
    PrinterParser<B> for Map<A, B, F, G, P>
{
    fn print(&self, i: B) -> Result<String, String> {
        let o = (self.g)(&i);
        self.parser.print(o)
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, B), String> {
        let (rem, a) = self.parser.parse(i)?;
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
    fn print(&self, i: B) -> Result<String, String> {
        let o = (self.g)(&i)?;
        self.parser.print(o)
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, B), String> {
        let (rem, a) = self.parser.parse(i)?;
        let mapped = (self.f)(a)?;
        Ok((rem, mapped))
    }
}

impl<A: Clone, B: Clone, PA: PrinterParser<A>, PB: PrinterParser<B>> PrinterParser<(A, B)>
    for ZipWith<A, B, PA, PB>
{
    fn print(&self, i: (A, B)) -> Result<String, String> {
        let (a, b) = i;
        let x = (self.a).print(a)?;
        let y = (self.b).print(b)?;
        Ok(x + &y)
    }

    fn parse<'a>(&self, i: &'a str) -> Result<(&'a str, (A, B)), String> {
        let (rem1, a) = self.a.parse(i)?;
        let (rem2, b) = self.b.parse(rem1)?;
        Ok((rem2, (a, b)))
    }
}

impl<A: Clone, P: PrinterParser<A>> PrinterParser<LinkedList<A>> for Rep<A, P> {
    fn print(&self, x: LinkedList<A>) -> Result<String, String> {
        x.into_iter()
            .map(|item| (self.parser).print(item))
            .collect::<Result<Vec<String>, String>>()
            .map(|vs| vs.join(""))
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
