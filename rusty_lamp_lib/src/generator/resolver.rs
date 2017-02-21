/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::ast::{BlockStatement, Statement, StatementKind};
use parser::program::Program;

pub struct Resolver {
    pub program: Program
}

#[derive(Debug)]
pub enum ResolveError {
    Resolve(String),
}

impl Resolver {
    pub fn new(program: Program) -> Resolver {
        Resolver {
            program: program
        }
    }

    pub fn resolve() -> Result<(), ResolveError> {
        return Ok();
    }
}
