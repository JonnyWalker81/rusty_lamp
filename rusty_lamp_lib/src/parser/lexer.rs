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

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0'
        }
        else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let tok = match self.ch {
            '=' => {
                Token::Equal
            },
            '+' => {
                let peek = self.peek_char();
                let interface_type = match peek {
                    'j' => Token::JavaInterface,
                    'o' => Token::ObjCInterface,
                    'c' => Token::CppInterface,
                    _ => Token::Illegal
                };

                if interface_type != Token::Illegal {
                    self.read_char();
                }

                interface_type
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
            '#' => Token::Comment(self.read_comment()),
            _ => {
                if Lexer::is_letter(self.ch) {
                    let ident_tok = self.read_identifier();
                    let tok = match ident_tok {
                        Token::Ident(ref s) => self.keywords.lookup_ident(s),
                        _ => ident_tok
                    };

                    return tok;
                }
                else if Lexer::is_digit(self.ch) {
                    return self.read_number();
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
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }

        // return self.input.chars().skip(position).take(self.position).unwrap();
        return String::from_str(&self.input[position..self.position]).unwrap();
    }
    
    fn read_number(&mut self) -> Token {
        let pos = self.position;

        let mut r = String::new();
        let mut is_float = false;
        while Lexer::is_digit(self.ch) {
            r.push(self.ch);
            if self.ch == '.' {
                is_float = true;
            }
            self.read_char();
        }

        return Token::Number(r);
    }

    fn read_comment(&mut self) -> String {
        let position = self.position + 1;

        loop {
            self.read_char();
            if Lexer::is_newline(self.ch) || self.ch == '\0' {
                // self.read_char();
                break;
            }
        }

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

    fn is_newline(ch: char) -> bool {
        return ch == '\n' || ch =='\r';
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
            '_' => true,
            _ => false
        };

        return result;
    }

    fn is_letter(ch: char) -> bool {
        let result = match ch {
            'a'...'z' => true,
            'A'...'Z' => true,
            '_' => true,
            _ => false
        };

        return result;
    }

    fn is_digit(ch: char) -> bool {
        return match ch {
            '0'...'9' | '.' => true,
            _ => false
        };
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
        let input = r#"@  import  enum (){}<>,; list<i32> map<string, i64>;
                       # this is a test of a comment
                           string date set<f32> record interface = +c{};"#;

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
            token_test_case{expected_token: Token::Comment(" this is a test of a comment".into()), expected_literal: "# this is a test of a comment".into()},
            token_test_case{expected_token: Token::Type(DataType::String, "string".into()), expected_literal: "string".into()},
            token_test_case{expected_token: Token::Type(DataType::Date, "date".into()), expected_literal: "date".into()},
            token_test_case{expected_token: Token::Type(DataType::Set, "set".into()), expected_literal: "set".into()},
            token_test_case{expected_token: Token::Lt, expected_literal: "<".into()},
            token_test_case{expected_token: Token::Type(DataType::Float32, "f32".into()), expected_literal: "f32".into()},
            token_test_case{expected_token: Token::Gt, expected_literal: ">".into()},
            token_test_case{expected_token: Token::Record, expected_literal: "record".into()},
            token_test_case{expected_token: Token::Interface, expected_literal: "interface".into()},
            token_test_case{expected_token: Token::Equal, expected_literal: "=".into()},
            token_test_case{expected_token: Token::CppInterface, expected_literal: "+c".into()},
            token_test_case{expected_token: Token::LBrace, expected_literal: "{".into()},
            token_test_case{expected_token: Token::RBrace, expected_literal: "}".into()},
            token_test_case{expected_token: Token::Semicolon, expected_literal: ";".into()},
            token_test_case{expected_token: Token::Eof, expected_literal: "".into()},
        ];

        let mut lexer = Lexer::new(input.into());

        for t in test_cases {
            let tok = lexer.next_token();

            // println!("{}", t.expected_literal);
            // println!("{}", tok);
            assert!(tok == t.expected_token, "token did not match: {} != {}", tok, t.expected_token);
        }
    }
}
