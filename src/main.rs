mod printerparser;

use std::{borrow::Borrow, collections::LinkedList, fmt::Display};

use crate::printerparser::PrinterParser;
use printerparser::*;

#[derive(Debug)]
enum JSON {
    Boolean(bool),
    Number(i64),
    String(String),
    Array(Vec<JSON>),
    Object(LinkedList<(String, JSON)>),
}

fn main() {
    let parse_true = string("true").map(|_| true, |_| ());
    let parse_false = string("false").map(|_| true, |_| ());

    let parse_boolean = (parse_true.or(parse_false)).map(
        |value| match value {
            Either::Left(left) => JSON::Boolean(left),
            Either::Right(right) => JSON::Boolean(right),
        },
        |value| match value {
            JSON::Boolean(true) => Either::Left(true),
            JSON::Boolean(false) => Either::Right(false),
            v => panic!("Unexpected value: {:#?}", v),
        },
    );

    let parse_number = repeat1(digit())
        .map(
            |digits| {
                digits
                    .into_iter()
                    .fold(0i64, |acc, val| acc * 10 + val.to_digit(10).unwrap() as i64)
            },
            |n| n.to_string().chars().collect(),
        )
        .map(
            |value| JSON::Number(value),
            |value| match value {
                JSON::Number(n) => *n,
                v => panic!("Unexpected value: {:#?}", v),
            },
        );

    let parse_string = surrounded_by(
        char('"'),
        ANY_CHAR.filter(|c| *c != '"').repeat(),
        char('"'),
    )
    .map(
        |chars| chars.into_iter().collect::<String>(),
        |string| string.chars().collect(),
    )
    .map(
        |value| JSON::String(value),
        |value| match value {
            JSON::String(s) => s.clone(),
            v => panic!("Unexpected value: {:#?}", v),
        },
    );

    let parse_json = parse_boolean.or(parse_number).or(parse_string).map(
        |value| match value {
            Either::Left(Either::Left(boolean)) => boolean,
            Either::Left(Either::Right(number)) => number,
            Either::Right(string) => string,
        },
        |json| match json {
            JSON::Boolean(b) => Either::Left(Either::Left(JSON::Boolean(*b))),
            JSON::Number(b) => Either::Left(Either::Right(JSON::Number(*b))),
            JSON::String(b) => Either::Right(JSON::String(b.clone())),
            _ => panic!(),
        },
    );

    let (_, boolean) = parse_json.parse("true", &mut ()).unwrap();
    let (_, string_t) = parse_json.parse("\"hello\"", &mut ()).unwrap();
    let (_, number_t) = parse_json.parse("123", &mut ()).unwrap();

    println!("{:?}", boolean);
    println!("{:?}", string_t);
    println!("{:?}", number_t)
}
