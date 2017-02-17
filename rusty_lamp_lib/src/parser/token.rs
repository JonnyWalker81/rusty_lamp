/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::fmt;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum DataType {
    None,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Float32,
    Float64,
    Bool,
    Map,
    List,
    String,
    Binary,
    Date,
    Set,
    Optional,
    Object(String)
}

impl DataType {
    pub fn new(name: String) -> DataType {
        match name {
            "i8" => DataType::Integer8,
            "i6"
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Token {
    Illegal,
    Eof,
    Import,
    Ident(String),
    Enum,
    Record,
    Interface,
    Colon,
    Comment,
    Semicolon,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Lt,
    Gt,
    Equal,
    StringToken(String),
    Type(DataType, String),
    True,
    False,
    AtSign
}

impl Token {
    pub fn to_str(&self) -> String {
        match *self {
            Token::Ident(ref s) => {
                s.clone()
            },
            _=> format!("{}", *self)
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Token::Illegal => "Illegal".into(),
            Token::Eof => "EOF".into(),
            Token::Import => "import".into(),
            Token::Ident(ref id) => format!("Ident({})", id),
            Token::Enum => "enum".into(),
            Token::Record => "record".into(),
            Token::Interface => "interface".into(),
            Token::Colon => ":".into(),
            Token::Comment => "#".into(),
            Token::Semicolon => ";".into(),
            Token::Comma => ",".into(),
            Token::LParen => "(".into(),
            Token::RParen => ")".into(),
            Token::LBrace => "{".into(),
            Token::RBrace => "}".into(),
            Token::Lt => "<".into(),
            Token::Gt => ">".into(),
            Token::Equal => "=".into(),
            Token::StringToken(ref s) => format!("{}", s),
            Token::Type(ref t, _) => format!("{}", t),
            Token::True => "true".into(),
            Token::False => "false".into(),
            Token::AtSign => "@".into(),
        };

        write!(f, "{}", printable)
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            DataType::None => "none",
            DataType::Integer8 => "i8",
            DataType::Integer16 => "i16",
            DataType::Integer32 => "i32",
            DataType::Integer64 => "i64",
            DataType::Float32 => "f32",
            DataType::Float64 => "f64",
            DataType::Binary => "binary",
            DataType::Bool => "bool",
            DataType::Date => "data",
            DataType::List => "list",
            DataType::Map => "map",
            DataType::Set => "set",
            DataType::String => "string",
            DataType::Optional => "optional",
            DataType::Object(_) => "object"
        };

        write!(f, "{}", printable)
    }
}
