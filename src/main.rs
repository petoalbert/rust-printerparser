mod printerparser;

use crate::printerparser::PrinterParser;
use printerparser::*;

#[derive(Debug, Clone)]
struct FunDef {
    name: String,
    args: Vec<String>,
}

fn main() {
    let name = as_string(
        ANY_CHAR
            .filter(|c| *c != '(' && *c != ',' && *c != ')')
            .repeat(),
    );
    let args = surrounded_by(
        string("("),
        separated_list(name.clone(), string(",")),
        string(")"),
    );
    let fun = preceded_by(string("fn "), name).zip_with(args).map(
        |(name, args)| FunDef {
            name: name,
            args: args.into_iter().collect(),
        },
        |f| (f.name.clone(), f.args.clone().into_iter().collect()),
    );

    let parsed = fun.parse("fn test(a,b,c)", &mut ()).unwrap().1;
    println!("Parsed: {:#?}", parsed);
    let printed = fun.print(parsed, &mut ()).unwrap();
    println!("Printed: {}", printed);
}
