/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */
use parser::ast::{Statement};
use parser::token::Token;
use std::vec::Vec;
use std::fmt;

#[derive(Clone)]
pub struct Program {
    pub statements: Vec<Statement>
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new()
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        let statements = self.statements.clone();
        for s in statements {
            println!("Statement: {}", s.stmtKind);
            let stmtStr = format!("{}", s.stmtKind);
            result.push_str(&stmtStr[..]);
        }

        write!(f, "{}", result)
    }
}

impl Default for Program {
    fn default() -> Program {
        Program {
            statements: Vec::new()
        }
    }
}
