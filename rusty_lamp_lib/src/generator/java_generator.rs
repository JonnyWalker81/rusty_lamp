/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::sync::Arc;
use std::io::{Write, BufWriter};
use std::fs::{File};
use parser::program::Program;
use parser::ast::{StatementKind};
use generator::generator::{ Generate };
use generator::spec::Spec;

pub struct JavaGenerator {
    
}

impl JavaGenerator {
    pub fn new() -> JavaGenerator {
        JavaGenerator{
            
        }
    }
}

impl Generate for JavaGenerator {
    // fn generate(&self, program: &Program) {
    //     println!("Generating Java files...");
    // }

    fn write_enum(&self, e: &StatementKind, spec: &Spec) {
        
    }

    fn write_record(&self, r: &StatementKind, spec: &Spec) {
        
    }

    fn write_interface(&self, i: &StatementKind) {
        
    }
}
