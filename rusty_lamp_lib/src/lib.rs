pub mod parser;
pub mod generator;
use parser::lexer::Lexer;
use parser::parser::Parser;
use parser::djinni_fmt::LampFmt;

use std::io::Write;

pub fn process(input: String, output: &mut Write) -> String {
    // let lexer = Lexer::new(input);
    // let mut parser = Parser::new(lexer);
    // let program = parser.parse_program().unwrap_or_default();

    let mut fmt = LampFmt::new(input.clone(), output);
    fmt.fmt();

    return String::new();
}

#[test]
fn it_works() {
}
