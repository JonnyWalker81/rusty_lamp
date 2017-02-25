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
    Interface(Token, Identifier, Vec<InterfaceType>, BlockStatement, Vec<DeriveType>),
    Record(Token, Identifier, BlockStatement, Vec<DeriveType>),
    Enum(Token, Identifier, BlockStatement),
    EnumMember(Token, Identifier),
    RecordMember(Token, Identifier, DataTypeStatement),
    Comment(Token, String),
    StringLiteral(Token, String),
    NumberLiteral(Token, String),
    Boolean(Token, bool),
    Block(BlockStatement),
    Const(Token, Identifier, DataTypeStatement, Arc<Statement>),
    Definition(Identifier, Arc<Statement>),
    Ident(Token, String),
    Function(Token, FunctionModifier, Identifier, Vec<Parameter>, DataTypeStatement)
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
            StatementKind::Interface(ref t, ref i, ref it, ref b, ref d) => {
                let mut result = String::new();

                let mut interface_types = Vec::new();
                for intfc_type in it {
                    interface_types.push(format!("{}", intfc_type));
                }

                result.push_str(format!("{} = interface ", i).as_str());
                result.push_str(interface_types.join(" ").as_str());
                result.push_str(" {");
                result.push_str(format!("{}", b).as_str());
                result.push_str("}");

                result
            },
            StatementKind::Record(ref t, ref i, ref b, ref d) => {
                let mut result = String::new();

                result.push_str(format!("{} = record {{", i).as_str());
                result.push_str(format!("{}", b).as_str());
                result.push_str("}");

                result
            },
            StatementKind::Enum(ref t, ref i, ref b) => {
                let mut result = String::new();

                result.push_str(format!("{} = enum {{\n", i).as_str());
                result.push_str(format!("\t{}\n", b).as_str());
                result.push_str("}");

                result
            },
            StatementKind::EnumMember(ref t, ref i) => {
                format!("{};", i)
            },
            StatementKind::RecordMember(ref t, ref i, ref d) => {
                format!("{}: {};", i, d)
            },
            StatementKind::Function(ref t, ref m, ref i, ref p, ref r) => {
                let mut result = String::new();

                result.push_str(format!("{} {}(", m, i).as_str());

                let mut parameters = Vec::new();
                for param in p {
                    parameters.push(format!("{}", param));
                }

                result.push_str(parameters.join(", ").as_str());
                result.push_str(format!("): {};", r).as_str());

                result
            },
            StatementKind::Const(ref t, ref i, ref dt, ref v) => {
                let mut result = String::new();

                result.push_str(format!("const {}: {} = {};", i, dt, v.stmtKind).as_str());

                result
            },
            StatementKind::Comment(ref t, ref s) => {
                format!("#{}", s)
            },
            StatementKind::Boolean(ref t, ref b) => {
                format!("{}", b)
            },
            StatementKind::NumberLiteral(ref t, ref s) => {
                format!("{}", s)
            },
            StatementKind::StringLiteral(ref t, ref s) => {
                format!("\"{}\"", s)
            },
            StatementKind::Ident(ref t, ref s) => {
                format!("{}", s)
            },
            StatementKind::Definition(ref i, ref s) => {
                format!("{} = {},", i, s.stmtKind)
            },
            StatementKind::Block(ref bs) => {
                format!("{{ {} }}", bs)
            }
        };

        write!(f, "{}", printable)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Parameter {
    pub ident: Identifier,
    pub data_type: DataTypeStatement
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!("({}: {})", self.ident, self.data_type);
        write!(f, "{}", result)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DeriveType {
    None,
    Ord,
    Eq
}

impl fmt::Display for DeriveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            DeriveType::None => "",
            DeriveType::Ord => "ord",
            DeriveType::Eq => "eq"
        };

        write!(f, "{}", printable)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum FunctionModifier {
    None,
    Static
}

impl fmt::Display for FunctionModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            FunctionModifier::Static => "static",
            _ => ""
        };

        write!(f, "{}", printable)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum InterfaceType {
    Java,
    ObjectiveC,
    Cpp
}

impl fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            InterfaceType::Java => "+j",
            InterfaceType::ObjectiveC => "+o",
            InterfaceType::Cpp => "+c"
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

    pub fn get_name(&self) -> String {
        match *self {
            DataTypeStatement::None => "none".into(),
            DataTypeStatement::String => "string".into(),
            DataTypeStatement::Integer8 => "i8".into(),
            DataTypeStatement::Integer16 => "i16".into(),
            DataTypeStatement::Integer32 => "i32".into(),
            DataTypeStatement::Integer64 => "i64".into(),
            DataTypeStatement::Float32 => "f32".into(),
            DataTypeStatement::Float64 => "f64".into(),
            DataTypeStatement::Binary => "binary".into(),
            DataTypeStatement::Date => "date".into(),
            DataTypeStatement::Bool => "bool".into(),
            DataTypeStatement::Set(..) => "set".into(),
            DataTypeStatement::List(..) => "list".into(),
            DataTypeStatement::Map(..) => "map".into(),
            DataTypeStatement::Object(ref i) => format!("{}", i.value)
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
