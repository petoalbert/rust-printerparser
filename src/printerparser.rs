use std::{collections::LinkedList, marker::PhantomData};

/*
Notes:

Not having a flat_map has some limitations: it would be difficult to parse using some context, e.g.
something like python/yaml where the indentation is increased at every level. We could add a state parameter
to print/parse to overcome that limitation, just like parserz does.

 */

// Helper functions

pub const ANY_CHAR: ConsumeChar = ConsumeChar;

// little endian number types

pub fn le_f32<S>() -> impl PrinterParserOps<S, f32> {
    bytes(4).map(
        |b| f32::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_f64<S>() -> impl PrinterParserOps<S, f64> {
    bytes(4).map(
        |b| f64::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i8<S>() -> impl PrinterParserOps<S, i8> {
    bytes(4).map(
        |b| i8::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i16<S>() -> impl PrinterParserOps<S, i16> {
    bytes(4).map(
        |b| i16::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i32<S>() -> impl PrinterParserOps<S, i32> {
    bytes(4).map(
        |b| i32::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_i64<S>() -> impl PrinterParserOps<S, i64> {
    bytes(4).map(
        |b| i64::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_u16<S>() -> impl PrinterParserOps<S, u16> {
    bytes(4).map(
        |b| u16::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_u32<S>() -> impl PrinterParserOps<S, u32> {
    bytes(4).map(
        |b| u32::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

pub fn le_u64<S>() -> impl PrinterParserOps<S, u64> {
    bytes(4).map(
        |b| u64::from_le_bytes(b.try_into().unwrap()),
        |&i| i.to_le_bytes().to_vec(),
    )
}

// big endian number types

pub fn be_i8<S>() -> impl PrinterParserOps<S, i8> {
    bytes(4).map(
        |b| i8::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_i16<S>() -> impl PrinterParserOps<S, i16> {
    bytes(4).map(
        |b| i16::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_i32<S>() -> impl PrinterParserOps<S, i32> {
    bytes(4).map(
        |b| i32::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_i64<S>() -> impl PrinterParserOps<S, i64> {
    bytes(4).map(
        |b| i64::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_u16<S>() -> impl PrinterParserOps<S, u16> {
    bytes(4).map(
        |b| u16::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_u32<S>() -> impl PrinterParserOps<S, u32> {
    bytes(4).map(
        |b| u32::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_u64<S>() -> impl PrinterParserOps<S, u64> {
    bytes(4).map(
        |b| u64::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_f32<S>() -> impl PrinterParserOps<S, f32> {
    bytes(4).map(
        |b| f32::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

pub fn be_f64<S>() -> impl PrinterParserOps<S, f64> {
    bytes(4).map(
        |b| f64::from_be_bytes(b.try_into().unwrap()),
        |&i| i.to_be_bytes().to_vec(),
    )
}

// endianness-independent number types

pub fn i8() -> impl PrinterParserOps<Endianness, i8> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i8()),
        Endianness::LittleEndian => Box::new(le_i8()),
    })
}

pub fn i16() -> impl PrinterParserOps<Endianness, i16> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i16()),
        Endianness::LittleEndian => Box::new(le_i16()),
    })
}

pub fn i32() -> impl PrinterParserOps<Endianness, i32> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i32()),
        Endianness::LittleEndian => Box::new(le_i32()),
    })
}

pub fn i64() -> impl PrinterParserOps<Endianness, i64> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_i64()),
        Endianness::LittleEndian => Box::new(le_i64()),
    })
}

pub fn u16() -> impl PrinterParserOps<Endianness, u16> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_u16()),
        Endianness::LittleEndian => Box::new(le_u16()),
    })
}

pub fn u32() -> impl PrinterParserOps<Endianness, u32> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_u32()),
        Endianness::LittleEndian => Box::new(le_u32()),
    })
}

pub fn u64() -> impl PrinterParserOps<Endianness, u64> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_u64()),
        Endianness::LittleEndian => Box::new(le_u64()),
    })
}

pub fn f32() -> impl PrinterParserOps<Endianness, f32> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_f32()),
        Endianness::LittleEndian => Box::new(le_f32()),
    })
}

pub fn f64() -> impl PrinterParserOps<Endianness, f64> {
    map_state(|e| match e {
        Endianness::BigEndindan => Box::new(be_f64()),
        Endianness::LittleEndian => Box::new(le_f64()),
    })
}

pub fn map_state<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>> + Clone>(
    f: F,
) -> impl PrinterParserOps<S, A> {
    MapState {
        f: f,
        phantom: PhantomData,
    }
}

struct MapState<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>>> {
    f: F,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>> + Clone> Clone for MapState<S, A, F> {
    fn clone(&self) -> Self {
        MapState {
            f: self.f.clone(),
            phantom: PhantomData,
        }
    }
}

impl<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>>> PrinterParser<S, A>
    for MapState<S, A, F>
{
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        (self.f)(s).write(i, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        (self.f)(s).read(i, s)
    }
}

impl<S, A, F: Fn(&mut S) -> Box<dyn PrinterParser<S, A>> + Clone> PrinterParserOps<S, A>
    for MapState<S, A, F>
{
}

pub enum Endianness {
    BigEndindan,
    LittleEndian,
}

#[allow(dead_code)]
pub fn digit<S>() -> impl PrinterParserOps<S, char> {
    ANY_CHAR.filter(|c| c.is_digit(10))
}

#[allow(dead_code)]
pub fn char<S>(c: char) -> impl PrinterParserOps<S, ()> {
    ANY_CHAR.filter(move |x| x == &c).map(|_| (), move |_| c)
}

pub fn bytes<S>(count: usize) -> impl PrinterParserOps<S, Vec<u8>> {
    ConsumeBytes(count)
}

pub fn string<'a, S>(s: &'a str) -> impl PrinterParserOps<S, ()> + 'a {
    ExpectString(s.as_bytes())
}

pub fn preceded_by<S, A: Clone, PA: PrinterParserOps<S, A>, PU: PrinterParserOps<S, ()>>(
    before: PU,
    parser: PA,
) -> impl PrinterParserOps<S, A> {
    before
        .zip_with(parser)
        .map(|(_, a)| a, |a| ((), (*a).clone()))
}

pub fn followed_by<S, A: Clone, PA: PrinterParserOps<S, A>, PU: PrinterParserOps<S, ()>>(
    parser: PA,
    after: PU,
) -> impl PrinterParserOps<S, A> {
    parser
        .zip_with(after)
        .map(|(a, _)| a, |a| ((*a).clone(), ()))
}

#[allow(dead_code)]
pub fn repeat1<S, A: Clone, PA: PrinterParserOps<S, A>>(
    combinator: PA,
) -> impl PrinterParserOps<S, LinkedList<A>> {
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
pub fn take_while<S, A, PA: PrinterParserOps<S, A>, F: Fn(&A) -> bool + Clone>(
    parser: PA,
    predicate: F,
) -> impl PrinterParserOps<S, LinkedList<A>> {
    parser.filter(predicate).repeat()
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
    PA: PrinterParserOps<S, A>,
    PU1: PrinterParserOps<S, ()>,
    PU2: PrinterParserOps<S, ()>,
>(
    before: PU1,
    parser: PA,
    after: PU2,
) -> impl PrinterParserOps<S, A> {
    followed_by(preceded_by(before, parser), after)
}

pub fn separated_list<S, A: Clone, PA: PrinterParserOps<S, A>, PU: PrinterParserOps<S, ()>>(
    parser: PA,
    sep: PU,
) -> impl PrinterParserOps<S, LinkedList<A>> {
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
pub fn as_string<S>(
    p: impl PrinterParserOps<S, LinkedList<char>>,
) -> impl PrinterParserOps<S, String> {
    p.map(
        |cs| cs.into_iter().collect(),
        |s: &String| s.chars().collect(),
    )
}

pub trait PrinterParserOps<S, A>
where
    Self: PrinterParser<S, A> + Clone,
{
    fn filter<F: Fn(&A) -> bool>(self, predicate: F) -> Filter<S, A, F, Self>
    where
        Self: Sized,
    {
        Filter {
            parser: self,
            predicate: predicate,
            phantom: PhantomData,
        }
    }

    fn map<B, F: Fn(A) -> B, G: Fn(&B) -> A>(self, f: F, g: G) -> Map<S, A, B, F, G, Self>
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

    fn map_result<B, F: Fn(A) -> Result<B, String>, G: Fn(&B) -> Result<A, String>>(
        self,
        f: F,
        g: G,
    ) -> MapResult<S, A, B, F, G, Self>
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

    fn zip_with<B, P: PrinterParser<S, B>>(self, other: P) -> ZipWith<S, A, B, Self, P>
    where
        Self: Sized,
    {
        ZipWith {
            a: self,
            b: other,
            phantom: PhantomData,
        }
    }

    fn repeat(self) -> Rep<S, A, Self>
    where
        Self: Sized,
    {
        Rep {
            parser: self,
            phantom: PhantomData,
        }
    }

    fn or<B, PB: PrinterParser<S, B>>(self, other: PB) -> Alt<S, A, B, Self, PB> {
        Alt {
            a: self,
            b: other,
            phantom: PhantomData,
        }
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

pub struct Alt<S, A, B, PA: PrinterParser<S, A>, PB: PrinterParser<S, B>> {
    a: PA,
    b: PB,
    phantom: PhantomData<(A, B, S)>,
}

impl<S, A, B, PA: PrinterParser<S, A> + Clone, PB: PrinterParser<S, B> + Clone> Clone
    for Alt<S, A, B, PA, PB>
{
    fn clone(&self) -> Self {
        Alt {
            a: self.a.clone(),
            b: self.b.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct Filter<S, A, F: Fn(&A) -> bool, P: PrinterParser<S, A>> {
    parser: P,
    predicate: F,
    phantom: PhantomData<(A, S)>,
}

impl<S, A, F: Fn(&A) -> bool + Clone, P: PrinterParser<S, A> + Clone> Clone for Filter<S, A, F, P> {
    fn clone(&self) -> Self {
        Filter {
            parser: self.parser.clone(),
            predicate: self.predicate.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct Map<S, A, B, F: Fn(A) -> B, G: Fn(&B) -> A, P: PrinterParser<S, A> + Sized> {
    parser: P,
    f: F,
    g: G,
    phantom: PhantomData<(A, B, S)>,
}

impl<
        S,
        A,
        B,
        F: Fn(A) -> B + Clone,
        G: Fn(&B) -> A + Clone,
        P: PrinterParser<S, A> + Sized + Clone,
    > Clone for Map<S, A, B, F, G, P>
{
    fn clone(&self) -> Self {
        Map {
            parser: self.parser.clone(),
            f: self.f.clone(),
            g: self.g.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct MapResult<
    S,
    A,
    B,
    F: Fn(A) -> Result<B, String>,
    G: Fn(&B) -> Result<A, String>,
    P: PrinterParser<S, A> + Sized,
> {
    parser: P,
    f: F,
    g: G,
    phantom: PhantomData<(A, B, S)>,
}

impl<
        S,
        A,
        B,
        F: Fn(A) -> Result<B, String> + Clone,
        G: Fn(&B) -> Result<A, String> + Clone,
        P: PrinterParser<S, A> + Sized + Clone,
    > Clone for MapResult<S, A, B, F, G, P>
{
    fn clone(&self) -> Self {
        MapResult {
            parser: self.parser.clone(),
            f: self.f.clone(),
            g: self.g.clone(),
            phantom: PhantomData,
        }
    }
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

pub struct Rep<S, A, P: PrinterParser<S, A>> {
    parser: P,
    phantom: PhantomData<(A, S)>,
}

impl<S, A, P: PrinterParser<S, A> + Clone> Clone for Rep<S, A, P> {
    fn clone(&self) -> Self {
        Rep {
            parser: self.parser.clone(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

// Parser implementations

impl<S, A, B, PA: PrinterParser<S, A>, PB: PrinterParser<S, B>> PrinterParser<S, Either<A, B>>
    for Alt<S, A, B, PA, PB>
{
    fn write(&self, i: Either<A, B>, s: &mut S) -> Result<Vec<u8>, String> {
        match i {
            Either::Left(a) => self.a.write(a, s),
            Either::Right(b) => self.b.write(b, s),
        }
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], Either<A, B>), String> {
        match self.a.read(i, s) {
            Ok((rem, a)) => Ok((rem, Either::Left(a))),
            Err(_) => match self.b.read(i, s) {
                Ok((rem, b)) => Ok((rem, Either::Right(b))),
                Err(e) => Err(e),
            },
        }
    }
}

impl<S, A, B, PA: PrinterParser<S, A> + Clone, PB: PrinterParser<S, B> + Clone>
    PrinterParserOps<S, Either<A, B>> for Alt<S, A, B, PA, PB>
{
}

impl<'a, S> PrinterParser<S, ()> for ExpectString<'a> {
    fn write(&self, _: (), s: &mut S) -> Result<Vec<u8>, String> {
        Ok(self.0.to_vec())
    }

    fn read<'b>(&self, i: &'b [u8], s: &mut S) -> Result<(&'b [u8], ()), String> {
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

impl<'a, S> PrinterParserOps<S, ()> for ExpectString<'a> {}

impl<S, A, F: Fn(&A) -> bool + Clone, P: PrinterParser<S, A>> PrinterParser<S, A>
    for Filter<S, A, F, P>
{
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        self.parser.write(i, s) // TODO
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        let (rem, a) = self.parser.read(i, s)?;
        if (self.predicate)(&a) {
            Ok((rem, a))
        } else {
            Err("Filter predicate didn't match".to_owned())
        }
    }
}

impl<S, A, F: Fn(&A) -> bool + Clone, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, A>
    for Filter<S, A, F, P>
{
}

impl<S> PrinterParser<S, Vec<u8>> for ConsumeBytes {
    fn write(&self, i: Vec<u8>, s: &mut S) -> Result<Vec<u8>, String> {
        Ok(i)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], Vec<u8>), String> {
        if (i.len() < self.0.into()) {
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
        std::str::from_utf8(&i[..1])
            .or_else(|_| std::str::from_utf8(&i[..2]))
            .or_else(|_| std::str::from_utf8(&i[..3]))
            .or_else(|_| std::str::from_utf8(&i[..4]))
            .map_err(|e| format!("{}", e))
            .map(|s| (&i[s.as_bytes().len()..], s.chars().nth(0).unwrap()))
    }
}

impl<S> PrinterParserOps<S, char> for ConsumeChar {}

impl<S, A, B, F: Fn(A) -> B + Clone, G: Fn(&B) -> A + Clone, P: PrinterParser<S, A>>
    PrinterParser<S, B> for Map<S, A, B, F, G, P>
{
    fn write(&self, i: B, s: &mut S) -> Result<Vec<u8>, String> {
        let o = (self.g)(&i);
        self.parser.write(o, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], B), String> {
        let (rem, a) = self.parser.read(i, s)?;
        Ok((rem, (self.f)(a)))
    }
}

impl<S, A, B, F: Fn(A) -> B + Clone, G: Fn(&B) -> A + Clone, P: PrinterParser<S, A> + Clone>
    PrinterParserOps<S, B> for Map<S, A, B, F, G, P>
{
}

impl<
        S,
        A,
        B,
        F: Fn(A) -> Result<B, String> + Clone,
        G: Fn(&B) -> Result<A, String> + Clone,
        P: PrinterParser<S, A>,
    > PrinterParser<S, B> for MapResult<S, A, B, F, G, P>
{
    fn write(&self, i: B, s: &mut S) -> Result<Vec<u8>, String> {
        let o = (self.g)(&i)?;
        self.parser.write(o, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], B), String> {
        let (rem, a) = self.parser.read(i, s)?;
        let mapped = (self.f)(a)?;
        Ok((rem, mapped))
    }
}

impl<
        S,
        A,
        B,
        F: Fn(A) -> Result<B, String> + Clone,
        G: Fn(&B) -> Result<A, String> + Clone,
        P: PrinterParser<S, A> + Clone,
    > PrinterParserOps<S, B> for MapResult<S, A, B, F, G, P>
{
}

impl<S, A, B, PA: PrinterParser<S, A>, PB: PrinterParser<S, B>> PrinterParser<S, (A, B)>
    for ZipWith<S, A, B, PA, PB>
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
    PrinterParserOps<S, (A, B)> for ZipWith<S, A, B, PA, PB>
{
}

impl<S, A, P: PrinterParser<S, A>> PrinterParser<S, LinkedList<A>> for Rep<S, A, P> {
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

impl<S, A, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, LinkedList<A>> for Rep<S, A, P> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_expected() {
        assert!(matches!(digit().parse("2", &mut ()), Ok(("", '2'))));
        assert!(matches!(digit().parse("a", &mut ()), Err(_)));
        assert_eq!(digit().print('2', &mut ()).unwrap(), "2")
    }

    #[test]
    fn test_string() {
        assert!(matches!(
            string("hello").parse("hello there", &mut ()),
            Ok((" there", ()))
        ));

        assert_eq!(string("hello").print((), &mut ()).unwrap(), "hello")
    }

    #[test]
    fn test_preceded_by() {
        let grammar = preceded_by(char('*'), string("hello"));
        assert!(matches!(grammar.parse("*hello", &mut ()), Ok(("", ()))));
        assert_eq!(string("*hello").print((), &mut ()).unwrap(), "*hello")
    }

    #[test]
    fn test_repeat() {
        let grammar = string("rust").repeat();

        let values = vec![(), (), ()];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("rustrustrust", &mut ()) {
            Ok(("", result)) => assert_eq!(result, list_for_parse),
            _ => panic!("Unexpected value"),
        }

        let values = vec![(), ()];
        let mut list_for_print = LinkedList::new();

        list_for_print.extend(values);

        assert_eq!(grammar.print(list_for_print, &mut ()).unwrap(), "rustrust")
    }

    #[test]
    fn test_many1() {
        let grammar = repeat1(string("rust"));
        let values = vec![(), ()];
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
    fn test_or() {
        let grammar = string("rust").or(string("haskell"));

        assert!(matches!(
            grammar.parse("rust", &mut ()),
            Ok(("", Either::Left(())))
        ));
        assert!(matches!(
            grammar.parse("haskell", &mut ()),
            Ok(("", Either::Right(())))
        ));
        assert!(matches!(grammar.parse("javascript", &mut ()), Err(_)));
        assert_eq!(grammar.print(Either::Left(()), &mut ()).unwrap(), "rust");
        assert_eq!(
            grammar.print(Either::Right(()), &mut ()).unwrap(),
            "haskell"
        );
    }

    #[test]
    fn test_take_while() {
        let grammar = take_while(ANY_CHAR, |a| a.is_digit(10));
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
    fn test_i32() {
        let bytes: [u8; 4] = [1, 2, 3, 4]; // 1 + 2*2^8 + 3*2^16 + 4*2^24 4 + 3*2^8 + 2*2^16 + 2^24

        let (_, i_le) = i32().read(&bytes, &mut Endianness::LittleEndian).unwrap();
        let (_, i_be) = i32().read(&bytes, &mut Endianness::BigEndindan).unwrap();
        let bytes_le = i32().write(i_le, &mut Endianness::LittleEndian).unwrap();
        let bytes_be = i32().write(i_be, &mut Endianness::BigEndindan).unwrap();

        assert_eq!(i_le, 67_305_985);
        assert_eq!(i_be, 16_909_060);
        assert_eq!(bytes_le, bytes);
        assert_eq!(bytes_be, bytes);
    }

    #[derive(Clone)]
    struct TestState {
        a: bool,
    }
}
