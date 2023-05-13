mod printerparser;

use printerparser::*;
use crate::printerparser::PrinterParser;

#[derive(Debug)]
struct FunDef { name: String, args: Vec<String> }

fn main() {
    // This is a function because PrinterParser doesn't implement copy, and I needed to reuse this.
    // Alternatively we could pass references to all combinator functions.
    let name = || as_string(any_char.filter(|c| *c != '(' && *c != ',' && *c != ')').repeat());
    let args = surrounded_by(string("("), separated_list(name, string(",")), string(")"));
    let fun = preceded_by(string("fn "), name()).zip_with(args).map(
        |(name, args)| FunDef { name: name, args: args.into_iter().collect() },
        |f| (f.name, f.args.clone().into_iter().collect())
    );
    
    let parsed = fun.parse("fn test(a,b,c)").unwrap().1;
    println!("Parsed: {:#?}", parsed);
    let printed = fun.print(parsed);
    println!("Printed: {}", printed);
}


