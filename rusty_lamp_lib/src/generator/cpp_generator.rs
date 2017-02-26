/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::sync::Arc;
use std::io::{Write, BufWriter};
use std::fs::{File};
use generator::generator::{Generate};
use generator::spec::Spec;
use parser::program::Program;
use parser::ast::{StatementKind};

pub struct CppGenerator {
    
}

impl CppGenerator {
    pub fn new() -> CppGenerator {
        CppGenerator {
            
        }
    }

    fn write_header(&self, w: &mut Write) {
        writeln!(w, "// AUTOGENERATED FILE - DO NOT MODIFY!");
        writeln!(w, "// This file was generated by rusty_lamp");

        writeln!(w, "#pragma once");
    }

    fn wrap_with_namespace<F>(&self, w: &mut Write, block: F) where F: Fn(&mut Write)  {
        writeln!(w, "namespace_gen {{");
        block(w);
        writeln!(w, "}}");
    }
}

impl Generate for CppGenerator {
    fn write_enum(&self, e: &StatementKind, spec: &Spec) {
        if let StatementKind::Enum(_, ref i, ref b) = *e {
            println!("Generating Enum: {} {{", i.value);
            let mut w = self.make_writer(spec, &i.value);
            self.write_header(&mut w);

            self.wrap_with_namespace(&mut w, |w| {
                writeln!(w, "enum {}", i.value);
                for o in &b.statements {
                    match o.stmtKind {
                        StatementKind::EnumMember(_, ref oi) => {
                            writeln!(w, "{};", oi.value);
                        },
                        _ => {}
                    }
                }
                writeln!(w, "}}");
            });
            
        }
    }

    fn write_record(&self, r: &StatementKind, spec: &Spec) {
        if let StatementKind::Record(_, ref i, ref bs, ref dt) = *r {
            println!("Generating Record: {}", i.value);
            let mut w = self.make_writer(spec, &i.value);

            self.write_header(&mut w);

            self.wrap_with_namespace(&mut w, |w| {
                writeln!(w, "struct {} {{", i.value);
                for f in &bs.statements {
                    match f.stmtKind {
                        StatementKind::RecordMember(_, ref id, ref dts) => {
                            writeln!(w, "{} {};", dts, id.value);
                        },
                        _ => {}
                    }
                }
                writeln!(w, "}};");
            });
        }
    }

    fn write_interface(&self, i: &StatementKind) {
        
    }

    // fn make_writer(&self, spec: &Spec, file_name: String) -> Write{
    //     // Open a file in write-only mode, returns `io::Result<File>`
    //     let path = format!("{}/{}/{}.hpp", spec.root, spec.cpp_root, file_name);
    //     let mut file = match File::create(&path) {
    //         Err(why) => panic!("couldn't create {}: {}",
    //                         path,
    //                         why.description()),
    //         Ok(file) => file,
    //     };

    //     file
    // }
}
