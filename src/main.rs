extern crate rusty_lamp_lib;

#[macro_use]
extern crate clap;

use clap::App;
use rusty_lamp_lib::parser;

use std::env;
use std::fs::File;
use std::io::{ Read, Write };
use std::io;

fn main() {
    println!("Hello, world!");

    let matches = clap_app!(rusty_lamp =>
                            (version: "0.1")
                            (author: "Jonathan Rothberg")
                            (about: "Djinni implementation in Rust")
                            (@arg INPUT: "Input Djinni IDL file")
    ).get_matches();

    match matches.value_of("INPUT") {
        Some(i) => {
            println!("{}", i);
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
        None => {
            
        }
    }
}
