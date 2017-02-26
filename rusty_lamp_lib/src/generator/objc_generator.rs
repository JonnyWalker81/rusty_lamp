/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::sync::Arc;
use std::io::{Write, BufWriter};
use std::fs::{File};
use parser::ast::{StatementKind};
use generator::generator::{ Generate };
use parser::program::Program;
use generator::spec::Spec;

pub struct ObjcGenerator {
    
}

impl ObjcGenerator {
    pub fn new() -> ObjcGenerator {
        ObjcGenerator {
            
        }
    }
}

impl Generate for ObjcGenerator {
    // fn generate(&self, program: &Program) {
    //     println!("Generating Objective-C files...");
    // }

    fn write_enum(&self, e: &StatementKind, spec: &Spec) {
        
    }

    fn write_record(&self, r: &StatementKind, spec: &Spec) {
        
    }

    fn write_interface(&self, i: &StatementKind) {
        
    }
}
