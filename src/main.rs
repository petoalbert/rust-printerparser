mod printerparser;

use std::collections::LinkedList;

use crate::printerparser::PrinterParser;
use printerparser::*;

#[derive(Debug, Clone)]
enum JSON {
    Boolean(bool),
    Number(i64),
    String(String),
    Array(LinkedList<JSON>),
    Object(LinkedList<(String, JSON)>),
}

fn whitespace() -> impl PrinterParserOps<(), ()> {
    ANY_CHAR
        .filter(|&c| c == ' ' || c == '\t' || c == '\n')
        .repeat()
        .map(|_| (), |_| LinkedList::new())
}

fn parse_boolean() -> impl PrinterParserOps<(), JSON> {
    let parse_true = string("true").map(|_| JSON::Boolean(true), |_| ());
    let parse_false = string("false").map(|_| JSON::Boolean(false), |_| ());

    parse_true.or(parse_false)
}

fn parse_number() -> impl PrinterParserOps<(), JSON> {
    repeat1(digit())
        .map(
            |digits| {
                digits
                    .into_iter()
                    .fold(0i64, |acc, val| acc * 10 + val.to_digit(10).unwrap() as i64)
            },
            |n| n.to_string().chars().collect(),
        )
        .map_result(
            |value, _| Ok(JSON::Number(value)),
            |value, _| match value {
                JSON::Number(n) => Ok(*n),
                v => Err(format!("Unexpected value in number: {:#?}", v)),
            },
        )
}

fn parse_string_literal() -> impl PrinterParserOps<(), String> {
    surrounded_by(
        char('"'),
        ANY_CHAR.filter(|c| *c != '"').repeat(),
        char('"'),
    )
    .map(
        |chars| chars.into_iter().collect::<String>(),
        |string| string.chars().collect(),
    )
}

fn parse_string() -> impl PrinterParserOps<(), JSON> {
    parse_string_literal().map_result(
        |value, _| Ok(JSON::String(value)),
        |value, _| match value {
            JSON::String(s) => Ok(s.clone()),
            v => Err(format!("Unexpected value in string: {:#?}", v)),
        },
    )
}

fn token<A: Clone, P: PrinterParserOps<(), A>>(p: P) -> impl PrinterParserOps<(), A> {
    surrounded_by(whitespace(), p, whitespace())
}

fn parse_array() -> impl PrinterParserOps<(), JSON> {
    defer(|| {
        Box::new(
            surrounded_by(
                token(char('[')),
                separated_list(parse_json(), token(char(','))),
                token(char(']')),
            )
            .map_result(
                |v, _| Ok(JSON::Array(v)),
                |value, _| match value {
                    JSON::Array(s) => Ok(s.clone()),
                    v => Err(format!("Unexpected value in array: {:#?}", v)),
                },
            ),
        )
    })
}

fn parse_object() -> impl PrinterParserOps<(), JSON> {
    defer(|| {
        Box::new(
            surrounded_by(
                token(char('{')),
                parse_string_literal()
                    .zip_with(surrounded_by(
                        token(char(':')),
                        parse_json(),
                        token(char(',')),
                    ))
                    .repeat(),
                token(char('}')),
            )
            .map_result(
                |pairs, _| Ok(JSON::Object(pairs)),
                |value, _| match value {
                    JSON::Object(s) => Ok(s.clone()),
                    v => Err(format!("Unexpected value in array: {:#?}", v)),
                },
            ),
        )
    })
}

fn parse_json() -> impl PrinterParserOps<(), JSON> {
    parse_boolean()
        .or(parse_number())
        .or(parse_string())
        .or(parse_array())
        .or(parse_object())
}

fn main() {
    // TODO: maybe
    let object_literal = "{ \
        \"this\": \"is valid JSON\", \
        \"almost\":  false, \
        \"year\": [2,0,2,3], \
    }";

    let (_, object_t) = parse_json().parse(object_literal, &mut ()).unwrap();

    println!("{:?}", object_t);

    // TODO: fix printing
}
