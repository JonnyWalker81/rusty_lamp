/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{Token, DataType};
use parser::lexer::Lexer;
use parser::ast::{BlockStatement, Statement, StatementKind, Identifier};
use parser::program::Program;
use std::fmt;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer: lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new()
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();

        while !self.cur_token_is(Token::Eof) {
            let stmt = self.parse_statement();
            match stmt {
                Some(s) => {
                    program.statements.push(s);
                },
                None => {
                    
                }
            }

            self.next_token();
        }
        return Some(program);
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Import => {
                return self.parse_import_statement();
            },
            _ => {
                return self.build_block_statements();
            }
        }
    }

    fn parse_import_statement(&mut self) -> Option<Statement> {
        return None;
    }

    fn build_block_statements(&mut self) -> Option<Statement> {
        return None;
    }

    fn peek_token_is_ident(&mut self) -> bool {
        if let Token::Ident(..) = self.peek_token {
            self.next_token();
            return true;
        }
        else {
            let t = self.peek_token.clone();
            println!("peek_error -> {}", t);
            self.peek_error(Token::Ident(String::from("")));
            return false;
        }
    }

    fn cur_token_is(&self, t: Token) -> bool {
        return self.cur_token == t;
    }

    fn peek_token_is(&self, t: Token) -> bool {
        return self.peek_token == t;
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        let checkToken = t.clone();
        if self.peek_token_is(checkToken) {
            self.next_token();
            return true;
        }
        else {
            self.peek_error(t);
            return false;
        }
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!("expected next token to be {}, got {} instead.", t, self.peek_token);
        self.errors.push(msg);
    }
}
