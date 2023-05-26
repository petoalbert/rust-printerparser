use std::rc::Rc;
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
#[derive(PartialEq, Debug)]
pub enum Endianness {
    BigEndindan,
    LittleEndian,
}

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

#[allow(dead_code)]
pub fn digit<S>() -> impl PrinterParserOps<S, char> {
    ANY_CHAR.filter(|c| c.is_digit(10))
}

#[allow(dead_code)]
pub fn char<S>(c: char) -> Default<S, char, MapResult<S, char, char, ConsumeChar>> {
    ANY_CHAR.filter(move |x| x == &c).default(c)
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

pub fn preceded_by<
    S,
    A: Clone,
    B,
    PA: PrinterParserOps<S, A>,
    PB: PrinterParserOps<S, B> + DefaultValue<S, B> + 'static,
>(
    before: PB,
    parser: PA,
) -> MapResult<S, (B, A), A, ZipWith<S, B, A, PB, PA>> {
    before.clone().zip_with(parser).map_result(
        |(_, a), _| Ok(a),
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
) -> MapResult<S, (A, B), A, ZipWith<S, A, B, PA, PB>> {
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

    combinator.zip_with(c2.repeat().clone()).map_result(
        |(a, mut aa), _| {
            aa.push_front(a);
            return Ok(aa);
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
) -> MapResult<S, (A, C), A, ZipWith<S, A, C, MapResult<S, (B, A), A, ZipWith<S, B, A, P1, PA>>, P2>>
{
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

pub trait DefaultValue<S, A> {
    fn value(&self, s: &S) -> Result<A, String>;
}

pub trait PrinterParserOps<S, A>
where
    Self: PrinterParser<S, A> + Clone,
{
    fn filter<F: Fn(&A) -> bool + 'static + Clone>(self, predicate: F) -> MapResult<S, A, A, Self>
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
                if predicate(&a) {
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
    ) -> MapResult<S, A, B, Self>
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
    ) -> MapResult<S, A, B, Self>
    where
        Self: Sized,
    {
        MapResult {
            parser: self,
            f: Rc::new(f),
            g: Rc::new(g),
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

    fn count(self, times: usize) -> Count<S, A, Self>
    where
        Self: Sized,
    {
        Count {
            times: times,
            parser: self.clone(),
            phantom: PhantomData,
        }
    }

    fn or<PB: PrinterParser<S, A>>(self, other: PB) -> Alt<S, A, Self, PB> {
        Alt {
            a: self,
            b: other,
            phantom: PhantomData,
        }
    }

    fn as_string(self) -> MapResult<S, A, String, Self>
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
    ) -> State<S, A, F, G, Self> {
        State {
            parser: self,
            read_state: read_state,
            write_state: write_state,
            phantom: PhantomData,
        }
    }

    fn default(self, a: A) -> Default<S, A, Self> {
        Default {
            parser: self,
            value: a,
            phantom: PhantomData,
        }
    }

    fn as_value<B: Clone + PartialEq + 'static>(self, b: B) -> MapResult<S, A, B, Self>
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

impl<S, A: Clone, P: PrinterParser<S, A> + Clone> Clone for Default<S, A, P> {
    fn clone(&self) -> Self {
        Default {
            parser: self.parser.clone(),
            value: self.value.clone(),
            phantom: PhantomData,
        }
    }
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
        F: Fn(A, &mut S) -> Result<(), String> + Clone,
        G: Fn(&S) -> Result<A, String> + Clone,
        P: PrinterParser<S, A> + Clone,
    > Clone for State<S, A, F, G, P>
{
    fn clone(&self) -> Self {
        State {
            parser: self.parser.clone(),
            read_state: self.read_state.clone(),
            write_state: self.write_state.clone(),
            phantom: PhantomData,
        }
    }
}

impl<
        S,
        A,
        F: Fn(A, &mut S) -> Result<(), String>,
        G: Fn(&S) -> Result<A, String>,
        P: PrinterParser<S, A>,
    > PrinterParser<S, ()> for State<S, A, F, G, P>
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
    > PrinterParserOps<S, ()> for State<S, A, F, G, P>
{
}

impl<
        S,
        A,
        F: Fn(A, &mut S) -> Result<(), String> + Clone,
        G: Fn(&S) -> Result<A, String> + Clone,
        P: PrinterParser<S, A> + Clone,
    > DefaultValue<S, ()> for State<S, A, F, G, P>
{
    fn value(&self, s: &S) -> Result<(), String> {
        Ok(())
    }
}

pub struct Defer<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>>> {
    resolve: F,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>> + Clone> Clone for Defer<S, A, F> {
    fn clone(&self) -> Self {
        Defer {
            resolve: self.resolve.clone(),
            phantom: PhantomData,
        }
    }
}

pub fn defer<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>>>(f: F) -> Defer<S, A, F> {
    Defer {
        resolve: f,
        phantom: PhantomData,
    }
}

impl<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>> + Clone> PrinterParserOps<S, A>
    for Defer<S, A, F>
{
}

impl<S, A, F: Fn() -> Box<dyn PrinterParser<S, A>> + Clone> PrinterParser<S, A> for Defer<S, A, F> {
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

impl<S, A, B, P: PrinterParser<S, A> + Sized + Clone> Clone for MapResult<S, A, B, P> {
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

impl<
        S,
        A,
        B,
        PA: PrinterParser<S, A> + Clone + DefaultValue<S, A>,
        PB: PrinterParser<S, B> + Clone + DefaultValue<S, B>,
    > DefaultValue<S, (A, B)> for ZipWith<S, A, B, PA, PB>
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

impl<S, A, P: PrinterParser<S, A> + Clone> Clone for Rep<S, A, P> {
    fn clone(&self) -> Self {
        Rep {
            parser: self.parser.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct Count<S, A, P: PrinterParser<S, A>> {
    times: usize,
    parser: P,
    phantom: PhantomData<(A, S)>,
}

impl<S, A, P: PrinterParser<S, A> + Clone> Clone for Count<S, A, P> {
    fn clone(&self) -> Self {
        Count {
            times: self.times,
            parser: self.parser.clone(),
            phantom: PhantomData,
        }
    }
}

// Parser implementations

impl<S, A, P: PrinterParser<S, A>> PrinterParser<S, A> for Default<S, A, P> {
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        self.parser.write(i, s)
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        self.parser.read(i, s)
    }
}

impl<S, A: Clone, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, A> for Default<S, A, P> {}

impl<S, A: Clone, P: PrinterParser<S, A>> DefaultValue<S, A> for Default<S, A, P> {
    fn value(&self, _: &S) -> Result<A, String> {
        Ok(self.value.clone())
    }
}

impl<S, A: Clone, PA: PrinterParser<S, A>, PB: PrinterParser<S, A>> PrinterParser<S, A>
    for Alt<S, A, PA, PB>
{
    fn write(&self, i: A, s: &mut S) -> Result<Vec<u8>, String> {
        self.a.write(i.clone(), s).or(self.b.write(i, s))
    }

    fn read<'a>(&self, i: &'a [u8], s: &mut S) -> Result<(&'a [u8], A), String> {
        self.a.read(i, s).or(self.b.read(i, s))
    }
}

impl<S, A: Clone, PA: PrinterParser<S, A> + Clone, PB: PrinterParser<S, A> + Clone>
    PrinterParserOps<S, A> for Alt<S, A, PA, PB>
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
        if i.len() < self.0.into() {
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
        if i.len() == 0 {
            return Err("0 length input encountered".to_owned());
        }

        std::str::from_utf8(&i[..1])
            .or_else(|_| std::str::from_utf8(&i[..2]))
            .or_else(|_| std::str::from_utf8(&i[..3]))
            .or_else(|_| std::str::from_utf8(&i[..4]))
            .map_err(|e| format!("{}", e))
            .map(|s| (&i[s.as_bytes().len()..], s.chars().nth(0).unwrap()))
    }
}

impl<S> PrinterParserOps<S, char> for ConsumeChar {}

impl<S, A, B, P: PrinterParser<S, A>> PrinterParser<S, B> for MapResult<S, A, B, P> {
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

impl<S, A, B, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, B> for MapResult<S, A, B, P> {}

impl<S, A, B, P: PrinterParser<S, A> + Clone + DefaultValue<S, A>> DefaultValue<S, B>
    for MapResult<S, A, B, P>
{
    fn value(&self, s: &S) -> Result<B, String> {
        let a = self.parser.value(s)?;
        (self.f)(a, s)
    }
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

impl<S, A, P: PrinterParser<S, A>> PrinterParser<S, LinkedList<A>> for Count<S, A, P> {
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

impl<S, A, P: PrinterParser<S, A> + Clone> PrinterParserOps<S, LinkedList<A>> for Count<S, A, P> {}

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
        let expected = "hello".to_owned();
        assert!(matches!(
            string("hello").parse("hello there", &mut ()),
            Ok((" there", expected))
        ));

        assert_eq!(
            string("hello").print("hello".to_owned(), &mut ()).unwrap(),
            "hello"
        );

        assert!(matches!(
            string("hello").parse("general kenobi", &mut ()),
            Err(_)
        ));
    }

    #[test]
    fn test_tag() {
        let expected = b"hello".to_vec();
        assert!(matches!(
            tag(b"hello").read(b"hello there", &mut ()),
            Ok((b" there", expected))
        ));

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
    fn test_preceded_by() {
        let grammar = preceded_by(char('*'), string("hello"));
        let expected = "hello".to_owned();
        assert_eq!(grammar.parse("*hello", &mut ()), Ok(("", expected)));
        assert_eq!(
            string("*hello")
                .print("*hello".to_owned(), &mut ())
                .unwrap(),
            "*hello"
        )
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

        assert!(matches!(
            grammar.parse("rust", &mut ()),
            Ok(("", parsed_rust))
        ));
        assert!(matches!(
            grammar.parse("haskell", &mut ()),
            Ok(("", parsed_haskell))
        ));
        assert!(matches!(grammar.parse("javascript", &mut ()), Err(_)));
        assert_eq!(grammar.print(parsed_rust, &mut ()).unwrap(), "rust");
        assert_eq!(grammar.print(parsed_haskell, &mut ()).unwrap(), "haskell");
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
    fn test_take_till() {
        let grammar = take_till(ANY_CHAR, |a| a.is_ascii_punctuation());
        let (rest, result) = grammar.parse("shut! up", &mut ()).unwrap();
        let mut list = LinkedList::new();
        list.extend(vec!['s', 'h', 'u', 't']);
        assert_eq!(rest, "! up");
        assert_eq!(result, list);

        let printed = grammar.print(list, &mut ()).unwrap();
        assert_eq!(printed, "shut")
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

    #[test]
    fn test_state() {
        let le_bytes: [u8; 5] = [0, 1, 2, 3, 4];
        let be_bytes: [u8; 5] = [1, 4, 3, 2, 1];

        let endianness = bytes(1).as_state(
            |bs, s: &mut Endianness| match bs.first().unwrap() {
                0 => {
                    *s = Endianness::LittleEndian;
                    Ok(())
                }
                1 => {
                    *s = Endianness::BigEndindan;
                    Ok(())
                }
                _ => Err("Unreadable endianness".to_owned()),
            },
            |s: &Endianness| match s {
                Endianness::LittleEndian => Ok([0].to_vec()),
                Endianness::BigEndindan => Ok([1].to_vec()),
            },
        );

        let grammar = preceded_by(endianness, i32());

        let mut parsed_state = Endianness::LittleEndian;
        let (_, i) = grammar.read(&be_bytes, &mut parsed_state).unwrap();
        let result_bytes = grammar.write(i, &mut Endianness::LittleEndian).unwrap();

        assert_eq!(parsed_state, Endianness::BigEndindan);
        assert_eq!(i, 67_305_985);
        assert_eq!(result_bytes, le_bytes);
    }
}
