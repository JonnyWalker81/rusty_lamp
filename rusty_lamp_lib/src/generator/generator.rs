/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::fs;
use std::fs::{File};
use std::io::{Write, BufWriter};
use std::error::Error;
use std::sync::Arc;
use std::convert::AsRef;
use parser::program::Program;
use parser::ast::{StatementKind};
use generator::spec::Spec;

pub trait Generate {
    // fn new() -> Arc<Generate> where Self:Sized;
    fn generate(&self, spec: &Spec, program: &Program) {
        for stmt in &program.statements {
            match stmt.stmtKind {
                StatementKind::Enum(..) => {
                    // let mut w = self.make_writer(spec, &i.value);
                    // write!(w, "before write_enum call...");
                    // w.flush();
                    // self.testW(&mut w);
                    self.write_enum(&stmt.stmtKind, &spec);
                    // writeln!(w, "after write_enum call...");
                    // w.flush();
                },
                StatementKind::Record(..) => {
                    self.write_record(&stmt.stmtKind, &spec);
                }
                _ => {}
            }
        }
    }
    fn testW(&self, w: &mut Write) {
        writeln!(w, "Test function...");
    }
    fn write_enum(&self, e: &StatementKind, spec: &Spec);
    fn write_record(&self, r: &StatementKind, spec: &Spec);
    fn write_interface(&self, i: &StatementKind);
    fn make_writer(&self, spec: &Spec, file_name: &String) -> BufWriter<File> {
        // Open a file in write-only mode, returns `io::Result<File>`
        let path = format!("{}/{}.hpp", spec.cpp_out_folder.clone().unwrap(), file_name);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                            path,
                            why.description()),
            Ok(file) => file,
        };

        BufWriter::new(file)

    }
    fn test(&self) {
    }
}

pub struct Generator<G: Generate>{
    gen: G,
}

impl<G: Generate> Generator<G> {
    pub fn new(gen: G) -> Generator<G> {
        Generator {
            gen: gen,
        }
    }

    pub fn generate(&self, spec: &Spec, program: &Program) {
        self.gen.generate(spec, program);
        
    }

    
}



