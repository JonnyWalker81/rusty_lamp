/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{Token, DataType};
use parser::lexer::Lexer;
use parser::ast::{BlockStatement, Statement, StatementKind,
                  Identifier, DataTypeStatement, InterfaceType,
                  Parameter, FunctionModifier, DeriveType };
use parser::program::Program;
use std::fmt;
use std::sync::Arc;

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
            // println!("{}", self.cur_token);
            self.next_token();
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let tok = self.cur_token.clone();
        match tok {
            Token::AtSign => {
                return self.parse_import_statement();
            },
            Token::Comment(ref s) => {
                return Some(self.parse_comment_statement());
            },
            _ => {
                return self.build_block_statements();
            }
        }
    }

    fn parse_comment_statement(&mut self) -> Statement {
        // println!("Comment: {}", self.cur_token);
        match self.cur_token {
            Token::Comment(ref s) => {
                return Statement{
                    stmtKind: StatementKind::Comment(self.cur_token.clone(), s.clone())
                };
            },
            _ => {
                return Statement::new();
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

        // println!("{}", ident_tok);
        // println!("{}", self.peek_token);
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
                },
                Token::Interface  => {
                    return Some(self.parse_interface_statement(ident_tok));
                },
                _ => {
                    return Some(Statement::new());
                }
            }
        }
        return None;
    }

    fn parse_enum_statement(&mut self, token: Token) -> Statement {
        let ident_tok = token;
        // println!("{}", ident_tok.to_str());
        
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
                Token::Comment(_) => {
                    block.statements.push(self.parse_comment_statement());
                }
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
        // println!("{}", ident_tok.to_str());

        self.next_token();

        let mut block = BlockStatement {token: Token::Record, statements: Vec::new() };
        while !self.cur_token_is(Token::RBrace) {
            let tok = self.cur_token.clone();
            match tok {
                Token::Ident(ref s) => {
                    if self.expect_peek(Token::Colon) {
                        self.next_token();
                        block.statements.push(Statement {
                            stmtKind: StatementKind::RecordMember(tok.clone(), Identifier {
                                token: tok.clone(),
                                value: s.clone()
                            }, self.parse_type())
                        });
                    }
                },
                Token::Comment(_) => {
                    block.statements.push(self.parse_comment_statement());
                },
                Token::Const => {
                    block.statements.push(self.parse_const_statement());
                },
                _ => {
                }
            }

            self.next_token();
        }

        let derived = self.parse_derives();

        return Statement {
            stmtKind: StatementKind::Record(Token::Record, Identifier {
                token: ident_tok.clone(),
                value: ident_tok.to_str()
            }, block, derived)
        }
    }

    fn get_ident_string(ident: Token) -> String {
        let ident_name = match ident {
            Token::Ident(ref s) => {
                s.clone()
            },
            _ => {
                "".into()
            }
        };

        ident_name
    }

    fn parse_const_statement(&mut self) -> Statement {
        let const_tok = self.cur_token.clone();

        self.next_token();

        let ident = self.cur_token.clone();

        let ident_name = match ident {
            Token::Ident(ref s) => {
                s.clone()
            },
            _ => {
                "".into()
            }
        };

        if !self.expect_peek(Token::Colon) {
            return Statement::new();
        }

        self.next_token();
        let const_type = self.parse_type();

        if !self.expect_peek(Token::Equal) {
            return Statement::new();
        }

        self.next_token();
        let value = self.parse_const_value();

        Statement {
            stmtKind: StatementKind::Const(const_tok, Identifier {
                token: ident,
                value: ident_name},
                                           const_type, Arc::new(value))
        }
    }

    fn parse_const_value(&mut self) -> Statement {
        let tok = self.cur_token.clone();

        match tok {
            Token::StringToken(ref s) => {
                return Statement {
                    stmtKind: StatementKind::StringLiteral(tok.clone(), s.clone())
                };
            },
            Token::True => {
                return Statement {
                    stmtKind: StatementKind::Boolean(tok.clone(), true)
                };
            },
            Token::False => {
                return Statement {
                    stmtKind: StatementKind::Boolean(tok.clone(), false)
                };
            },
            Token::Number(ref s) => {
                return Statement {
                    stmtKind: StatementKind::NumberLiteral(tok.clone(), s.clone())
                };
            },
            Token::LBrace => {
                return self.parse_const_block();
            },
            _ => {}
        }

        println!("Parsing cont value, should not get here...");
        Statement::new()
    }

    fn parse_const_block(&mut self) -> Statement {
        self.next_token();

        
        let mut block_statements = Vec::new();
        while !self.cur_token_is(Token::RBrace) {
            println!("cur_tok: {}", self.cur_token);
            let ident = self.cur_token.clone();

            if !self.expect_peek(Token::Equal) {
                return Statement::new();
            }

            self.next_token();
            
            let value = self.parse_const_value();
            let definition = Statement{ stmtKind: StatementKind::Definition(Identifier{
                token: ident.clone(),
                value: Parser::get_ident_string(ident) }, Arc::new(value))
            };

            block_statements.push(definition);

            if self.peek_token_is(Token::Comma) {
                self.next_token();
            }

            self.next_token();
        }

        Statement {
            stmtKind: StatementKind::Block(BlockStatement {
                token: Token::Const,
                statements: block_statements
            })
        }
    }

    fn parse_type(&mut self) -> DataTypeStatement {
        let type_name = self.cur_token.clone();

        // println!("type_name: {}", type_name);

        if self.peek_token_is(Token::Lt) {
            self.next_token();
            self.next_token();
            let tok = self.cur_token.clone();
            // println!("Type Token: {}", self.cur_token);
            let data_type = match type_name {
                Token::Type(ref d, ref s) => {
                    // println!("d: {}", d);
                    match *d {
                        DataType::Map => {
                            let key = self.parse_generic_type(&tok);
                            if self.expect_peek(Token::Comma) {
                                // println!("Map Comma: {}", self.cur_token);
                                self.next_token();
                                let value_tok = self.cur_token.clone();
                                let value = self.parse_generic_type(&value_tok);
                                DataTypeStatement::Map(Arc::new(key), Arc::new(value))
                            }
                            else {
                                DataTypeStatement::None
                            }
                        },
                        DataType::List => {
                            // println!("List Type: {}", tok);
                            DataTypeStatement::List(Arc::new(self.parse_generic_type(&tok)))
                        },
                        DataType::Set => {
                            // println!("Set Type: {}", tok);
                            DataTypeStatement::Set(Arc::new(self.parse_generic_type(&tok)))
                        }
                        _ => {
                            DataTypeStatement::from_data_type(&d)
                        }
                    }
                },
                Token::Ident(ref s) => {
                    // println!("Object Type: {}", tok);
                    DataTypeStatement::Object(Identifier{ token: tok.clone(), value: s.clone()})
                },
                _ => {
                    DataTypeStatement::None
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
                    DataTypeStatement::from_data_type(&d)
                },
                Token::Ident(ref s) => {
                    DataTypeStatement::Object(Identifier{token: type_name.clone(), value: s.clone()})
                },
                _ => {
                    DataTypeStatement::None
                }
            };
            return data_type;
        }

        if !self.expect_peek(Token::Semicolon) {
            // TODO: Return error
        }

        DataTypeStatement::None
    }

    fn parse_generic_type(&mut self, tok: &Token) -> DataTypeStatement {
        let result = match *tok {
            Token::Type(ref tt, ref ss) => {
                match *tt {
                    DataType::List | DataType::Set | DataType::Map => {
                        self.parse_type()
                    },
                    _ => {
                        DataTypeStatement::from_data_type(&tt)
                    }
                }
            },
            Token::Ident(ref s) => {
                DataTypeStatement::Object(Identifier{ token: tok.clone(), value: s.clone()})
            },
            _ => {
                DataTypeStatement::None
            }
        };

        result
    }

    fn parse_interface_statement(&mut self, ident: Token) -> Statement {
        let ident_tok = ident;
        // println!("ident: {}", ident_tok.to_str());

        self.next_token();

        // println!("next: {}", self.cur_token);

        let mut interface_types = Vec::new();
        while !self.cur_token_is(Token::LBrace) {
            match self.cur_token {
                Token::JavaInterface => {
                    interface_types.push(InterfaceType::Java);
                },
                Token::ObjCInterface => {
                    interface_types.push(InterfaceType::ObjectiveC);
                },
                Token::CppInterface => {
                    interface_types.push(InterfaceType::Cpp);
                },
                _ => {
                }
            }

            self.next_token();
        }

        let mut modifier = FunctionModifier::None;
        let mut block = BlockStatement {token: Token::Record, statements: Vec::new() };
        while !self.cur_token_is(Token::RBrace) {
            let tok = self.cur_token.clone();
            match tok {
                Token::Static => {
                    modifier = FunctionModifier::Static;
                },
                Token::Ident(ref s) => {
                    if self.expect_peek(Token::LParen) {
                        self.next_token();
                        block.statements.push(Statement {
                            stmtKind: StatementKind::Function(tok.clone(), modifier.clone(), Identifier {
                                token: tok.clone(),
                                value: s.clone()
                            }, self.parse_parameters(), self.parse_return_type())
                        });
                    }
                },
                Token::Comment(_) => {
                    block.statements.push(self.parse_comment_statement());
                },
                Token::Const => {
                    block.statements.push(self.parse_const_statement());
                },
                _ => {
                }
            }

            self.next_token();
        }

        let derived = self.parse_derives();

        return Statement {
            stmtKind: StatementKind::Interface(Token::Record, Identifier {
                token: ident_tok.clone(),
                value: ident_tok.to_str()
            }, interface_types, block, derived)
        }
    }

    fn parse_derives(&mut self) -> Vec<DeriveType> {
        let derived = if self.peek_token_is(Token::Dervive) {
            self.next_token();

            println!("{}", self.peek_token);
            if self.expect_peek(Token::LParen) {
                let mut derives = Vec::new();
                self.next_token();
                println!("next token: {}", self.cur_token);

                while !self.cur_token_is(Token::RParen) {
                    match self.cur_token {
                        Token::Eq => {
                            derives.push(DeriveType::Eq)
                        },
                        Token::Ord => {
                            derives.push(DeriveType::Ord)
                        },
                        _ => {
                        }
                    }

                    if self.peek_token_is(Token::Comma) {
                        self.next_token();
                    }

                    self.next_token();
                }

                derives
            }
            else {
                Vec::new()
            }
        }
        else {
            Vec::new()
        };

        derived
    }

    fn parse_parameters(&mut self) -> Vec<Parameter> {
        let mut parameters = Vec::new();

        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return parameters;
        }

        // self.next_token();

        let mut cur_tok = self.cur_token.clone();
        while !self.cur_token_is(Token::RParen) {
            // println!("cur_tok: {}", self.cur_token);
            cur_tok = self.cur_token.clone();
            match cur_tok {
                Token::Ident(ref s) => {
                    let id = Identifier {token: cur_tok.clone(), value: s.clone()};
                    if self.expect_peek(Token::Colon) {
                        self.next_token();
                        let t = self.parse_type();
                        let p = Parameter {
                            ident: id,
                            data_type: t
                        };

                        parameters.push(p);

                        if self.peek_token_is(Token::Comma) {
                            self.next_token();
                        }
                    }
                },
                _ => {
                }
            }

            self.next_token();
        }

        parameters
    }

    fn parse_return_type(&mut self) -> DataTypeStatement {
        self.next_token();
        // println!("return tok: {}", self.cur_token);
        if self.cur_token_is(Token::Colon) {
            self.next_token();
            let t = self.parse_type();
            if self.expect_peek(Token::Semicolon) {
                return t;
            }
        }
        DataTypeStatement::None
    }

    fn peek_token_is_ident(&mut self) -> bool {
        if let Token::Ident(..) = self.peek_token {
            self.next_token();
            return true;
        }
        else {
            let t = self.peek_token.clone();
            // println!("peek_error -> {}", t);
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
        let input = r#"@import "dep.djinni"
                       #comment test
                      "#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap_or_default();

        for s in program.statements.clone() {
            println!("{}", s.stmtKind);
        }

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
                // println!("{}", b);
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
                            set_list : list<set<string>>;
                            images: image_store;

                       }"#;

        // let input = r#"my_record = record {
        //                     hash: map<string, i32>;
        //                     store: image_store;
        //                }"#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap_or_default();

        // for s in program.statements {
        //     println!("{}", s.stmtKind);
        // }

        let test_cases = vec![
            TestData {expected_ident: "id".into(), expected_type: "i32".into()},
            TestData {expected_ident: "info".into(), expected_type: "string".into()},
            TestData {expected_ident: "store".into(), expected_type: "set<string>".into()},
            TestData {expected_ident: "hash".into(), expected_type: "map<string, i32>".into()},
            TestData {expected_ident: "values".into(), expected_type: "list<another_record>".into()},
            TestData {expected_ident: "set_list".into(), expected_type: "list<set<string>>".into()},
            TestData {expected_ident: "images".into(), expected_type: "image_store".into()},
        ];


        let stmt = program.statements[0].clone();
        match stmt.stmtKind {
            StatementKind::Record(ref t, ref i, ref b, ref d) => {
                assert!(i.value == "my_record", "{} != {}", i.value, "my_enum");
                // println!("{}", b);
                let mut index = 0;
                let block = b.clone();
                for s in block.statements {
                    match s.stmtKind {
                        StatementKind::RecordMember(ref t, ref i, ref d) => {
                            assert!(i.value == test_cases[index].expected_ident, "record mumber did not match: {} != {}", i.value, test_cases[index].expected_ident);
                            let t = format!("{}", d);
                            assert!(t == test_cases[index].expected_type, "types do not match: {} != {}", t, test_cases[index].expected_type);
                            // println!("DataType: {}", d);
                            index = index + 1;
                        },
                        _ => {
                            
                        }
                    }
                }
                
            },
            _ => {
                assert!(false, "exptected Record statement, got={}.", stmt.stmtKind);
            }
        }
    }

    #[test]
    fn test_interface_statement() {
                //         my_cpp_interface = interface +c {
                //     method_returning_nothing(value: i32);
                //     method_returning_some_type(key: string): another_record;
                // }
        struct TestData {
            expected_ident: String,
        }

        let input = r#"
                my_cpp_interface = interface +c {
                    #method with no return value
                    method_returning_nothing(value: i32);

                    method_multiple_params(first: string, value: i32): list<string>;
                    # Comments can also be put here
                    method_returning_some_type(key: string): another_record;
                    const string_const: string = "Constants can be put here";
                    static get_version(): i32;
                    const version: i32 = 1;
                    const min_value: another_record = {
                        key1 = 0,
                        key2 = ""
                    };
                }
                    "#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap_or_default();

        let test_cases = vec![
            TestData{expected_ident: "#method with no return value".into()},
            TestData{expected_ident: "method_returning_nothing".into()},
            TestData{expected_ident: "method_multiple_params".into()},
            TestData{expected_ident: "# Comments can also be put here".into()},
            TestData{expected_ident: "method_returning_some_type".into()},
            TestData{expected_ident: "string_const".into()},
            TestData{expected_ident: "get_version".into()},
            TestData{expected_ident: "version".into()},
            TestData{expected_ident: "min_value".into()},
        ];

        // println!("Number of Statements: {}", program.statements.len());
        for s in program.statements.clone() {
            // println!("{}", s.stmtKind);
        }

        let stmt = program.statements[0].clone();
        match stmt.stmtKind {
            StatementKind::Interface(ref t, ref i, ref it, ref b, ref d) => {
                assert!(i.value == "my_cpp_interface", "Interface name did not match: {} != {}", i.value, "my_cpp_interface");

                let mut index = 0;
                for bs in b.statements.clone() {
                    match bs.stmtKind {
                        StatementKind::Function(ref t, ref m, ref i, ref p, ref d) => {
                            assert!(i.value == test_cases[index].expected_ident, "Identifier did not match: {} != {}", i.value, test_cases[index].expected_ident);
                            index = index + 1;
                        },
                        StatementKind::Comment(ref t, ref s) => {
                            let comment = format!("{}", t);
                            assert!(comment == test_cases[index].expected_ident, "Comment did not match: {} != {}", comment, test_cases[index].expected_ident);
                            index = index + 1;
                        },
                        StatementKind::Const(ref t, ref i, ref dt, ref v) => {
                            let const_ident = format!("{}", i.value);
                            println!("{}", bs.stmtKind);
                            assert!(const_ident == test_cases[index].expected_ident, "Comment did not match: {} != {}", const_ident, test_cases[index].expected_ident);
                            index = index + 1;
                        },
                        _ => {
                            assert!(false, "Expected a function, comment or const statement, got={}", bs.stmtKind);
                        }
                    }
                }
            },
            _ => {
                assert!(false, "exptected Interface statement, got={}.", stmt.stmtKind);
            }
        }
    }
}
