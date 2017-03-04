extern crate rusty_lamp_lib;

#[macro_use]
extern crate clap;

use clap::{ App, Arg };
use rusty_lamp_lib::parser;

use std::env;
use std::fs::File;
use std::io::{ Read, Write };
use std::io;

fn main() {
    println!("Hello, world!");

    let matches = App::new("rusty_lamp").about("Djinni implmentation in Rust")
        .author("Jonathan Rothberg")
        .arg(Arg::with_name("IDL")
             .help("The IDF file with the type definitions, typically with extension \".djinni\".")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("FMT")
             .help("Format a Djinni file.")
             .takes_value(true)
             .required(false))
        .arg(Arg::with_name("include_path")
             .help("An include path to search for Djinni @import directives. Can specify multiple paths.")
             .takes_value(true)
             .long("idl-include-path"))
        .arg(Arg::with_name("java_out")
             .help("The output for the Java files (Generator disabled if unspecified.)")
             .takes_value(true)
             .long("java-out"))
        .arg(Arg::with_name("java_package")
             .help("The package name to use for generated Java classes.")
             .takes_value(true)
             .long("java-package"))
        .arg(Arg::with_name("java_class_access_modifier")
             .help("The access modifier to use for generated Java classes.")
             .takes_value(true)
             .long("java-class-access-modifier")
             .default_value("public"))
        .arg(Arg::with_name("java_cpp_exception")
             .help("The type for translated C++ exceptions in Java")
             .takes_value(true)
             .long("java-cpp-exception")
             .default_value("java.lang.RuntimeException this is not checked"))
        .arg(Arg::with_name("java_annotation")
             .help("Java annotation (@Foo) to place on all generated Java classes")
             .takes_value(true)
             .long("java-annotation"))
        .arg(Arg::with_name("java_nullable_annotation")
             .help("Java annotation (@Nullable) to place all fields and return values that are optional.")
             .long("java-nullable-annotation"))
        .arg(Arg::with_name("java_nonnull_annotation")
             .help("Java annotation (@Notnull) to place on all fields and return values that are not optional")
             .long("java-nonnull-annotation"))
        .arg(Arg::with_name("java_use_final_for_record")
             .help("Whether generated Java classes for records should be marked 'final'")
             .long("java-use-final-for-record")
             .takes_value(true)
             .default_value("true")).get_matches();

    // let matches = clap_app!(rusty_lamp =>
    //                         (version: "0.1")
    //                         (author: "Jonathan Rothberg")
    //                         (about: "Djinni implementation in Rust")
    //                         (@arg IDL: +required "The IDL file with the type definitions, typically with extension \".djinni\".")
    //                         (@arg FMT: --fmt "Format a Djinni file")
    //                         (@arg include_path: --("idl-include-path") "An include path to search for Djinni @import directives. Can specify multiple paths.")
    //                         (@arg java_out: --("java-out") "The output for the Java files (Generator disabled if unspecified)")
    // ).get_matches();

    if let Some(ref ann) = matches.value_of("java_use_final_for_record") {
        println!("Package: {:?}", ann.parse::<bool>());
    }

    match matches.value_of("INPUT") {
        Some(i) => {
            match matches.occurrences_of("FMT") {
                1 => {
                    let cwd = match env::var("PWD") {
                        Ok(c) => c,
                        Err(_) => String::new()
                    };
                    println!("Working Directory: {}", cwd);

                    let mut file = File::open(i).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let mut stdout = io::stdout();

                    rusty_lamp_lib::process(contents, &mut stdout);
                },
                _ => {
                    rusty_lamp_lib::compile(i.into());
                }
            }
        },
        None => {
            
        }
    }
}
