/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{Token, DataType};
use std::fmt;
// use std::fmt::write;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Statement {
    pub stmtKind: StatementKind
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum StatementKind {
    Import(Token, String),
    Interface(Token, Identifier, BlockStatement),
    Record(Token, Identifier, BlockStatement),
    Enum(Token, Identifier, BlockStatement),
    EnumMember(Token, Identifier),
    RecordMember(Token, Identifier, DataType),
    Function(Token, Identifier, Vec<Statement>, DataType)
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            StatementKind::Import(ref t, ref s) => {
                format!("@import {}", s)
            },
            StatementKind::Interface(ref t, ref i, ref b) => {
                let mut result = String::new();

                result.push_str(format!("{} = interface {{", i).as_str());
                result.push_str(format!("{}", b).as_str());
                result.push_str("}");

                result
            },
            StatementKind::Record(ref t, ref i, ref b) => {
                let mut result = String::new();

                result.push_str(format!("{} = record {{", i).as_str());
                result.push_str(format!("{}", b).as_str());
                result.push_str("}");

                result
            },
            StatementKind::Enum(ref t, ref i, ref b) => {
                let mut result = String::new();

                result.push_str(format!("{} = enum {{", i).as_str());
                result.push_str(format!("{}", b).as_str());
                result.push_str("}");

                result
            },
            StatementKind::EnumMember(ref t, ref i) => {
                format!("{};", i)
            },
            StatementKind::RecordMember(ref t, ref i, ref d) => {
                format!("{}: {};", i, d)
            },
            StatementKind::Function(ref t, ref i, ref s, ref r) => {
                let mut result = String::new();

                result.push_str(format!("{}(", i).as_str());

                let mut parameters = Vec::new();
                for stmt in s {
                    parameters.push(format!("{}", stmt.stmtKind));
                }

                result.push_str(parameters.join(", ").as_str());
                result.push_str(format!("): {};", r).as_str());

                result
            }
        };

        write!(f, "{}", printable)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>
}


#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        let stmts = self.statements.clone();
        for s in stmts {
            let stmt = format!("{}", s.stmtKind);
            result.push_str(&stmt[..]);
        }

        write!(f, "{}", result)
    }
}
