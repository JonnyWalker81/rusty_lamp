/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{ Token, DataType };

pub struct Keywords;

impl Keywords {
    pub fn new() -> Keywords {
        return Keywords{}
    }

    pub fn lookup_ident(&self, ident: &String) -> Token {
        match ident.as_ref() {
            "enum" => Token::Enum,
            "record" => Token::Record,
            "interface" => Token::Interface,
            "import" => Token::Import,
            "static" => Token::Static,
            "const" => Token::Const,
            "deriving" => Token::Dervive,
            "true" => Token::True,
            "false" => Token::False,
            "eq" => Token::Eq,
            "ord" => Token::Ord,
            "i8" => Token::Type(DataType::Integer8, "i8".into()),
            "i16" => Token::Type(DataType::Integer16, "i16".into()),
            "i32" => Token::Type(DataType::Integer32, "i32".into()),
            "i64" => Token::Type(DataType::Integer64, "i64".into()),
            "f32" => Token::Type(DataType::Float32, "f32".into()),
            "f64" => Token::Type(DataType::Float64, "f64".into()),
            "list" => Token::Type(DataType::List, "list".into()),
            "map" => Token::Type(DataType::Map, "map".into()),
            "set" => Token::Type(DataType::Set, "set".into()),
            "date" => Token::Type(DataType::Date, "date".into()),
            "binary" => Token::Type(DataType::Binary, "binary".into()),
            "bool" => Token::Type(DataType::Bool, "bool".into()),
            "optional" => Token::Type(DataType::Optional, "optional".into()),
            "string" => Token::Type(DataType::String, "string".into()),
            _ => Token::Ident(ident.clone())
        }
    }
}
