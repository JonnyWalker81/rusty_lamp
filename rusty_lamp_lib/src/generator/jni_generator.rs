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

pub struct JniGenerator {
    
}

impl JniGenerator {
    pub fn new() -> JniGenerator {
        JniGenerator {
            
        }
    }
}

impl Generate for JniGenerator {
    // fn generate(&self, program: &Program) {
    //     println!("Generating JNI files...");
    // }

    fn write_enum(&self, e: &StatementKind, spec: &Spec) {
        
    }

    fn write_record(&self, r: &StatementKind, spec: &Spec) {
        
    }

    fn write_interface(&self, i: &StatementKind) {
        
    }

}
