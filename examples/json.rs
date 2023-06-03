use parserprinter::printer_parser::combinator::*;
use parserprinter::printer_parser::primitives::*;
use parserprinter::printer_parser::printerparser::*;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
enum JSON {
    Boolean(bool),
    Number(i64),
    String(String),
    Array(Vec<JSON>),
    Object(Vec<(String, JSON)>),
}

fn whitespace() -> impl PrinterParserOps<(), String> + DefaultValue<(), String> {
    consume_char
        .filter(|&c| c == ' ' || c == '\t' || c == '\n')
        .repeat()
        .into_string()
        .default("".to_owned())
}

fn parse_boolean() -> impl PrinterParserOps<(), JSON> {
    let parse_true = string("true").into_value(JSON::Boolean(true));
    let parse_false = string("false").into_value(JSON::Boolean(false));

    parse_true.or(parse_false)
}

fn parse_number() -> impl PrinterParserOps<(), JSON> {
    repeat1(digit()).into_string().map_result(
        |value, _| {
            value
                .parse::<i64>()
                .map_err(|_| "Could not parse".to_owned())
                .map(JSON::Number)
        },
        |value, _| match value {
            JSON::Number(v) => Ok(v.to_string()),
            v => Err(format!("Unexpected value in number: {:#?}", v)),
        },
    )
}

fn parse_string_literal() -> impl PrinterParserOps<(), String> {
    surrounded_by(
        char('"'),
        consume_char.filter(|c| *c != '"').repeat(),
        char('"'),
    )
    .into_string()
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

fn token<A: Clone, P: PrinterParserOps<(), A> + DefaultValue<(), A>>(
    p: P,
) -> impl PrinterParserOps<(), A> + DefaultValue<(), A> {
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
                separated_list(
                    parse_string_literal()
                        .zip_with(token(char(':')))
                        .zip_with(parse_json())
                        .map(
                            |((key, _), json)| (key, json),
                            |(key, json)| ((key.to_owned(), ':'), json.to_owned()),
                        ),
                    token(char(',')),
                ),
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
        \"almost\":  true, \
        \"year\": [2,0,2,3] \
    }";

    let (_, object_t) = parse_json().parse(object_literal, &mut ()).unwrap();

    println!("{:?}", object_t);

    let printed = parse_json().print(&object_t, &mut ()).unwrap();
    println!("{:?}", printed)
}
