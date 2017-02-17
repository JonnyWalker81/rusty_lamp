/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{Token, DataType};
use parser::lexer::Lexer;
use parser::ast::{BlockStatement, Statement, StatementKind, Identifier, DataTypeStatement};
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

        // self.print_tokens();
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

    fn print_tokens(&mut self) {
        for _ in 0..5 {
            println!("{}", self.cur_token);
            self.next_token();
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::AtSign => {
                return self.parse_import_statement();
            },
            _ => {
                return self.build_block_statements();
            }
        }
    }

    fn parse_import_statement(&mut self) -> Option<Statement> {
        let tok = self.cur_token.clone();

        if self.cur_token_is(Token::AtSign) {
            self.next_token();
            let import_tok = self.cur_token.clone();

            if self.cur_token_is(Token::Import) {
                self.next_token();
                let import_literal = self.cur_token.clone();
                let literal = match import_literal {
                    Token::StringToken(ref s) => {
                        s.clone()
                    },
                    _ => {
                        "".into()
                    }
                };
                
                return Some(Statement {
                    stmtKind: StatementKind::Import(import_tok, literal)
                })
            }
        }

        return None;
    }

    fn build_block_statements(&mut self) -> Option<Statement> {
        let ident_tok = self.cur_token.clone();

        println!("{}", ident_tok);
        println!("{}", self.peek_token);
        if self.peek_token_is(Token::Equal) {
            self.next_token();
            self.next_token();

            let tok = self.cur_token.clone();
            match tok {
                Token::Enum => {
                    if self.peek_token_is(Token::LBrace) {
                        return Some(self.parse_enum_statement(ident_tok));
                    }
                },
                Token::Record => {
                    if self.peek_token_is(Token::LBrace) {
                        return Some(self.parse_record_statement(ident_tok));
                    }
                }
                _ => {
                    return Some(Statement::new());
                }
            }
        }
        return None;
    }

    fn parse_enum_statement(&mut self, token: Token) -> Statement {
        let ident_tok = token;
        println!("{}", ident_tok.to_str());
        
        self.next_token();

        let mut block = BlockStatement {token: Token::Enum, statements: Vec::new() };
        while !self.cur_token_is(Token::RBrace) {
            let tok = self.cur_token.clone();
            match tok {
                Token::Ident(ref s) => {
                    if self.peek_token_is(Token::Semicolon) {
                        block.statements.push(Statement {
                            stmtKind: StatementKind::EnumMember(tok.clone(), Identifier {
                                token: tok.clone(),
                                value: s.clone()
                            })
                        });
                    }
                },
                _ => {
                    
                }
            }

            self.next_token();
        }

        return Statement {
            stmtKind: StatementKind::Enum(Token::Enum, Identifier {
                token: ident_tok.clone(),
                value: ident_tok.to_str()
            }, block)
        }
    }

    fn parse_record_statement(&mut self, token: Token) -> Statement {
        let ident_tok = token;
        println!("{}", ident_tok.to_str());

        self.next_token();

        let mut block = BlockStatement {token: Token::Record, statements: Vec::new() };
        while !self.cur_token_is(Token::RBrace) {
            let tok = self.cur_token.clone();
            match tok {
                Token::Ident(ref s) => {
                    if self.peek_token_is(Token::Colon) {
                        block.statements.push(Statement {
                            stmtKind: StatementKind::RecordMember(tok.clone(), Identifier {
                                token: tok.clone(),
                                value: s.clone()
                            }, self.parse_type())
                        });
                    }
                },
                _ => {
                    
                }
            }

            self.next_token();
        }

        return Statement {
            stmtKind: StatementKind::Enum(Token::Enum, Identifier {
                token: ident_tok.clone(),
                value: ident_tok.to_str()
            }, block)
        }
    }

    fn parse_type(&mut self) -> DataTypeStatement {
        self.next_token();
        self.next_token();

        let type_name = self.cur_token.clone();

        println!("{}", type_name);

        if self.peek_token_is(Token::Lt) {
            self.next_token();
            self.next_token();
            let tok = self.cur_token.clone();
            println!("Type Token: {}", self.cur_token);
            let data_type = match tok {
                Token::Type(ref d, ref s) => {
                    println!("d: {}", d);
                    match *d {
                        DataType::Map => {
                            
                        },
                        DataType::List | DataType::Set => {

                        }
                        _ => {
                            DataTypeStatement::from_data_type(&d)
                        }
                    }
                },
                Token::Ident(ref s) => {
                    DataType::Object(s.clone())
                },
                _ => {
                    DataType::None
                }
            };

            if !self.expect_peek(Token::Gt) {
                // TODO: Return error
                return DataTypeStatement::None;
            }
            
            return data_type;
        }
        else {
            let data_type = match type_name {
                Token::Type(ref d, ref s) => {
                    d.clone()
                },
                Token::Ident(ref s) => {
                    DataType::Object(s.clone())
                },
                _ => {
                    DataType::None
                }
            };
            return data_type;
        }

        if !self.expect_peek(Token::Semicolon) {
            // TODO: Return error
        }

        DataType::None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_import() {
        let input = r#"@import "dep.djinni""#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap_or_default();

        // for s in program.statements {
        //     println!("{}", s.stmtKind);
        // }

        let stmt = program.statements[0].clone();
        match stmt.stmtKind {
            StatementKind::Import(ref t, ref s) => {
                assert!(s == "dep.djinni");
            },
            _ => {
                assert!(false, "exptected Import statement, got={}.", stmt.stmtKind);
            }
        }
    }

    #[test]
    fn test_parse_enum() {
        let input = r#"my_enum = enum {
                          option1;
                          option2;
                          option3;
                       }"#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap_or_default();

        // for s in program.statements {
        //     println!("{}", s.stmtKind);
        // }

        let expected_enum_values = vec!["option1", "option2", "option3"];

        let stmt = program.statements[0].clone();
        match stmt.stmtKind {
            StatementKind::Enum(ref t, ref i, ref b) => {
                assert!(i.value == "my_enum", "{} != {}", i.value, "my_enum");
                println!("{}", b);
                let mut index = 0;
                let block = b.clone();
                for s in block.statements {
                    match s.stmtKind {
                        StatementKind::EnumMember(ref t, ref i) => {
                            assert!(i.value == expected_enum_values[index], "enum mumber did not match: {} != {}", i.value, expected_enum_values[index]);
                            index = index + 1;
                        },
                        _ => {
                            
                        }
                    }
                }
                
            },
            _ => {
                assert!(false, "exptected Enum statement, got={}.", stmt.stmtKind);
            }
        }
    }

    #[test]
    fn test_parse_record() {
        struct TestData {
            expected_ident: String,
            expected_type: String
        }

        let input = r#"my_record = record {
                            id: i32;
                            info: string;
                            store: set<string>;
                            hash: map<string, i32>;

                            values: list<another_record>;
                       }"#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap_or_default();

        // for s in program.statements {
        //     println!("{}", s.stmtKind);
        // }

        let test_cases = vec![
            TestData {expected_ident: "id".into(), expected_type: "i32".into()},
            TestData {expected_ident: "info".into(), expected_type: "i32".into()},
            TestData {expected_ident: "store".into(), expected_type: "i32".into()},
            TestData {expected_ident: "hash".into(), expected_type: "i32".into()},
            TestData {expected_ident: "values".into(), expected_type: "i32".into()},
        ];


        let stmt = program.statements[0].clone();
        match stmt.stmtKind {
            StatementKind::Record(ref t, ref i, ref b) => {
                assert!(i.value == "my_record", "{} != {}", i.value, "my_enum");
                println!("{}", b);
                let mut index = 0;
                let block = b.clone();
                for s in block.statements {
                    match s.stmtKind {
                        StatementKind::RecordMember(ref t, ref i, ref d) => {
                            assert!(i.value == test_cases[index].expected_ident, "record mumber did not match: {} != {}", i.value, test_cases[index].expected_ident);
                            index = index + 1;
                        },
                        _ => {
                            
                        }
                    }
                }
                
            },
            _ => {
                assert!(false, "exptected Import statement, got={}.", stmt.stmtKind);
            }
        }
    }
}
