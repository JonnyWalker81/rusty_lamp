/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::ast::{BlockStatement, Statement, StatementKind};
use parser::program::Program;

pub struct Resolver {
}

#[derive(Debug)]
pub enum ResolveError {
    Resolve(String),
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
        }
    }

    pub fn resolve(&mut self, program: &Program) -> Result<(), ResolveError> {
        return Ok(());
    }
}
