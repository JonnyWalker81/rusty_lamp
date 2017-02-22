pub mod parser;
pub mod generator;
use parser::lexer::Lexer;
use parser::parser::Parser;
use parser::program::Program;
use generator::resolver::Resolver;
use parser::djinni_fmt::LampFmt;

use std::io::{ Read, Write };
use std::fs::File;
use std::env;



pub fn process(input: String, output: &mut Write) -> String {
    // let lexer = Lexer::new(input);
    // let mut parser = Parser::new(lexer);
    // let program = parser.parse_program().unwrap_or_default();

    let mut fmt = LampFmt::new(input.clone(), output);
    fmt.fmt();

    return String::new();
}

pub fn compile(main_file: String) {
    println!("Parsing...");
    let program = parse_file(main_file);

    // println!("Statement Count: {}", program.statements.len());
    // for s in program.statements {
    //     println!("{}", s.stmtKind);
    // }

    println!("Resolving...");
    let mut resolver = Resolver::new();
    resolver.resolve(&program);
}

fn parse_file(file: String) -> Program {
    let p = env::current_dir().unwrap();
    let full_path = format!("{}/{}", p.display(), file);
    println!("Path: {}", full_path);
    let mut file = File::open(full_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lexer = Lexer::new(contents);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Some(p) => {
            return p;
        },
        None => {
            panic!("Error parsing Djinni file...");
        }
    }
}

#[test]
fn it_works() {
}
