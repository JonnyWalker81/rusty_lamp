/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{ Token, DataType };
use std::ops::Index;
use std::string::String;
use std::str::FromStr;
use parser::keywords::Keywords;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    keywords: Keywords
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
            keywords: Keywords::new()
       };

        l.read_char();

        return l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        }
        else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let tok = match self.ch {
            '=' => {
                Token::Equal
            },
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\0' => Token::Eof,
            '"' => Token::StringToken(self.read_string()),
            ':' => Token::Colon,
            '@' => Token::AtSign,
            _ => {
                if Lexer::is_alphanumeric(self.ch) {
                    let ident_tok = self.read_identifier();
                    let tok = match ident_tok {
                        Token::Ident(ref s) => self.keywords.lookup_ident(s),
                        _ => ident_tok
                    };

                    return tok;
                }
                else {
                    return Token::Illegal;
                }
            }
        };

        self.read_char();

        return tok;
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;

        loop {
            self.read_char();
            if self.ch == '"' {
                break;
            }
        }

        // return self.input.chars().skip(position).take(self.position).unwrap();
        return String::from_str(&self.input[position..self.position]).unwrap();
    }

    fn read_identifier(&mut self) -> Token {
        let pos = self.position;

        let mut result = String::new();
        while Lexer::is_alphanumeric(self.ch) {
            result.push(self.ch);
            self.read_char();
        }

        return Token::Ident(result);
    }

    fn is_whitespace(&self, ch: char) -> bool {
       return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
    }

    fn skip_whitespace(&mut self) {
        loop {
            if !self.is_whitespace(self.ch) {
                break
            }

            self.read_char();
        }
    }

    fn is_alphanumeric(ch: char) -> bool {
        let result = match ch {
            'a'...'z' => true,
            'A'...'Z' => true,
            '0'...'9' => true,
            // '_' => true,
            _ => false
        };

        return result;
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    struct token_test_case {
        expected_token: Token,
        expected_literal: String
    }

    #[test]
    fn test_next_token() {
        let input = "@  import  enum (){}<>,; list<i32> map<string, i64>; string date set<f32> record interface;";

        let test_cases = vec![
            token_test_case{expected_token: Token::AtSign, expected_literal: "@".into()},
            token_test_case{expected_token: Token::Import, expected_literal: "import".into()},
            token_test_case{expected_token: Token::Enum, expected_literal: "enum".into()},
            token_test_case{expected_token: Token::LParen, expected_literal: "(".into()},
            token_test_case{expected_token: Token::RParen, expected_literal: ")".into()},
            token_test_case{expected_token: Token::LBrace, expected_literal: "{".into()},
            token_test_case{expected_token: Token::RBrace, expected_literal: "}".into()},
            token_test_case{expected_token: Token::Lt, expected_literal: "<".into()},
            token_test_case{expected_token: Token::Gt, expected_literal: ">".into()},
            token_test_case{expected_token: Token::Comma, expected_literal: ",".into()},
            token_test_case{expected_token: Token::Semicolon, expected_literal: ";".into()},
            token_test_case{expected_token: Token::Type(DataType::List, "list".into()), expected_literal: "list".into()},
            token_test_case{expected_token: Token::Lt, expected_literal: "<".into()},
            token_test_case{expected_token: Token::Type(DataType::Integer32, "i32".into()), expected_literal: "i32".into()},
            token_test_case{expected_token: Token::Gt, expected_literal: ">".into()},
            token_test_case{expected_token: Token::Type(DataType::Map, "map".into()), expected_literal: "map".into()},
            token_test_case{expected_token: Token::Lt, expected_literal: "<".into()},
            token_test_case{expected_token: Token::Type(DataType::String, "string".into()), expected_literal: "string".into()},
            token_test_case{expected_token: Token::Comma, expected_literal: ",".into()},
            token_test_case{expected_token: Token::Type(DataType::Integer64, "i64".into()), expected_literal: "i64".into()},
            token_test_case{expected_token: Token::Gt, expected_literal: ">".into()},
            token_test_case{expected_token: Token::Semicolon, expected_literal: ";".into()},
            token_test_case{expected_token: Token::Type(DataType::String, "string".into()), expected_literal: "string".into()},
            token_test_case{expected_token: Token::Type(DataType::Date, "date".into()), expected_literal: "date".into()},
            token_test_case{expected_token: Token::Type(DataType::Set, "set".into()), expected_literal: "set".into()},
            token_test_case{expected_token: Token::Lt, expected_literal: "<".into()},
            token_test_case{expected_token: Token::Type(DataType::Float32, "f32".into()), expected_literal: "f32".into()},
            token_test_case{expected_token: Token::Gt, expected_literal: ">".into()},
            token_test_case{expected_token: Token::Record, expected_literal: "record".into()},
            token_test_case{expected_token: Token::Interface, expected_literal: "interface".into()},
            token_test_case{expected_token: Token::Semicolon, expected_literal: ";".into()},
            token_test_case{expected_token: Token::Eof, expected_literal: "".into()},
        ];

        let mut lexer = Lexer::new(input.into());

        for t in test_cases {
            let tok = lexer.next_token();

            println!("{}", t.expected_literal);
            assert!(tok == t.expected_token, "token did not match {} != {}", tok, t.expected_literal);
        }
    }
}
