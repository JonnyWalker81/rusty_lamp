#![feature(conservative_impl_trait)]

pub mod parser;
pub mod generator;
use parser::lexer::Lexer;
use parser::parser::Parser;
use parser::program::Program;
use generator::resolver::Resolver;
use generator::generator::{ Generator, Generate};
use generator::cpp_generator::{CppGenerator};
use generator::java_generator::{JavaGenerator};
use generator::jni_generator::{JniGenerator};
use generator::objc_generator::{ObjcGenerator};
use generator::spec::Spec;
use parser::djinni_fmt::LampFmt;

use std::io::{ Read, Write };
use std::fs;
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

pub fn compile(main_file: String, spec: &Spec) {
    println!("Parsing...");
    let program = parse_file(main_file);

    // println!("Statement Count: {}", program.statements.len());
    // for s in program.statements {
    //     println!("{}", s.stmtKind);
    // }

    println!("Resolving...");
    let mut resolver = Resolver::new(spec.typer.clone());
    let result = resolver.resolve(&program);
    match result {
        Err(err) => {
            println!("Error: {:?}", err);
        },
        Ok(typer) => {
            println!("Generating...");
            // let spec = Spec::new("generated-src".into(), "cpp".into(), typer);
            setup_directories(spec);
            let mut cpp_generator = Generator::new(CppGenerator::new());
            cpp_generator.generate(spec, &program);

            let mut java_generator = Generator::new(JavaGenerator::new());
            java_generator.generate(spec, &program);

            let mut jni_generator = Generator::new(JniGenerator::new());
            jni_generator.generate(spec, &program);

            let mut objc_generator = Generator::new(ObjcGenerator::new());
            objc_generator.generate(spec, &program);
            // generator.generate::<JavaGenerator>(&program);
            // generator.generate::<JniGenerator>(&program);
            // generator.generate::<ObjcGenerator>(&program);
        }
    }

}

fn setup_directories(spec: &Spec) {
    fs::remove_dir_all("generated-src").unwrap_or_default();
    fs::create_dir_all(format!("{}/{}", spec.root, spec.cpp_root)).unwrap_or_default();
    fs::create_dir_all("generated-src/java").unwrap_or_default();
    fs::create_dir_all("generated-src/objc").unwrap_or_default();
}


fn parse_file(file: String) -> Program {
    let p = env::current_dir().unwrap();
    let full_path = format!("{}/{}", p.display(), file);
    // println!("Path: {}", full_path);
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
