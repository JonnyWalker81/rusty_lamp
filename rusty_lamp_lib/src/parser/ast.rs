/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{Token, DataType};
use std::fmt;
use std::sync::Arc;
// use std::fmt::write;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Statement {
    pub stmtKind: StatementKind
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            stmtKind: StatementKind::Noop
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum StatementKind {
    Noop,
    Import(Token, String),
    Interface(Token, Identifier, BlockStatement),
    Record(Token, Identifier, BlockStatement),
    Enum(Token, Identifier, BlockStatement),
    EnumMember(Token, Identifier),
    RecordMember(Token, Identifier, DataTypeStatement),
    Function(Token, Identifier, Vec<Statement>, DataType)
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            StatementKind::Noop => {
                "noop".into()
            }
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

pub enum PrimitiveTypes {
    string,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DataTypeStatement {
    None,
    String,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Float32,
    Float64,
    Binary,
    Date,
    Bool,
    Set(Arc<DataTypeStatement>),
    List(Arc<DataTypeStatement>),
    Map(Arc<DataTypeStatement>, Arc<DataTypeStatement>),
    Object(Identifier)
}

impl DataTypeStatement {
    pub fn from_data_type(dt: &DataType) -> DataTypeStatement {
        match *dt {
            DataType::Binary => DataTypeStatement::Binary,
            DataType::Bool => DataTypeStatement::Bool,
            DataType::Date => DataTypeStatement::Date,
            DataType::String => DataTypeStatement::String,
            DataType::Integer8 => DataTypeStatement::Integer8,
            DataType::Integer16 => DataTypeStatement::Integer16,
            DataType::Integer32 => DataTypeStatement::Integer32,
            DataType::Integer64 => DataTypeStatement::Integer64,
            DataType::Float32 => DataTypeStatement::Float32,
            DataType::Float64 => DataTypeStatement::Float64,
            _ => DataTypeStatement::None
        }
    }
}

impl fmt::Display for DataTypeStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            DataTypeStatement::None => "none".into(),
            DataTypeStatement::Binary => "binary".into(),
            DataTypeStatement::Bool => "bool".into(),
            DataTypeStatement::Date => "date".into(),
            DataTypeStatement::Float32 => "f32".into(),
            DataTypeStatement::Float64 => "f64".into(),
            DataTypeStatement::Integer8 => "i8".into(),
            DataTypeStatement::Integer16 => "i16".into(),
            DataTypeStatement::Integer32 => "i32".into(),
            DataTypeStatement::Integer64 => "i64".into(),
            DataTypeStatement::String => "string".into(),
            DataTypeStatement::Set(ref dt) => {
                format!("set<{}>", dt)
            },
            DataTypeStatement::List(ref dt) => {
                format!("list<{}>", dt)
            },
            DataTypeStatement::Map(ref k, ref v) => {
                format!("map<{}, {}>", k, v)
            },
            DataTypeStatement::Object(ref i) => {
                format!("{}", i)
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
