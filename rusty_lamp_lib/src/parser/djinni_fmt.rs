/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::io::Write;

use parser::parser::Parser;
use parser::lexer::Lexer;
use parser::ast::{Statement, StatementKind, BlockStatement, FunctionModifier, DeriveType };

pub struct LampFmt<'a> {
    input: String,
    output: &'a mut Write
}

impl<'a> LampFmt<'a> {
    pub fn new(input: String, output: &'a mut Write) -> LampFmt {
        LampFmt {
            input: input,
            output: output
        }
    }

    pub fn fmt(&'a mut self) {
        let lexer = Lexer::new(self.input.clone());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap_or_default();

        let indent = 0;
        for stmt in program.statements {
            self.print_statement(&stmt, indent);
        }
    }

    fn print_statement(&mut self, stmt: &Statement, indent: i32) {
        self.print_statement_kind(&stmt.stmtKind, indent);
    }

    fn print_statement_kind(&mut self, stmt_kind: &StatementKind, indent: i32) {
        match *stmt_kind {
            StatementKind::Enum(..) => {
                self.print_enum_statement(stmt_kind, indent);
                self.print_newline();
            },
            StatementKind::EnumMember(..) => {
               self.print_enum_member(stmt_kind, indent);
            },
            StatementKind::Record(..) => {
                self.print_record_statement(stmt_kind, indent);
                self.print_newline();
            },
            StatementKind::RecordMember(..) => {
                self.print_record_member(stmt_kind, indent);
            },
            StatementKind::Comment(..) => {
                self.print_comment_statement(stmt_kind, indent);
            },
            StatementKind::Interface(..) => {
                self.print_interface_statement(stmt_kind, indent);
                self.print_newline();
            },
            StatementKind::Function(..) => {
                self.print_function_statement(stmt_kind, indent);
            },
            StatementKind::Import(..) => {
                self.print_import_statement(stmt_kind);
                self.print_newline();
            },
            StatementKind::Const(..) => {
                self.print_const_statement(stmt_kind, indent);
            },
            StatementKind::StringLiteral(..) => {
                self.print_string_literal(stmt_kind, indent);
            },
            StatementKind::Block(..) => {
                self.print_block(stmt_kind, indent);
            },
            _ => {
            }
        }
    }

    fn print_enum_statement(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::Enum(_, ref i, ref b) = *stmt_kind {
            writeln!(self.output, "{} = enum {{", i.value);
            self.print_block_statement(b, indent);
            writeln!(self.output, "}}");
        }
    }

    fn print_enum_member(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::EnumMember(_, ref i) = *stmt_kind {
            self.print_spaces(indent);

            writeln!(self.output, "{};", i.value);
        }
    }

    fn print_record_statement(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::Record(_, ref i, ref b, ref d) = *stmt_kind {
            writeln!(self.output, "{} = record {{", i.value);
            self.print_block_statement(b, indent);
            write!(self.output, "}}");

            self.print_deriving(d);
        }
    }

    fn print_record_member(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::RecordMember(_, ref i, ref dt) = *stmt_kind {
            self.print_spaces(indent);

            writeln!(self.output, "{}: {};", i.value, dt);
        }
    }

    fn print_interface_statement(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::Interface(_, ref i, ref p, ref b, ref d) = *stmt_kind {
            write!(self.output, "{} = interface ", i.value);
            for it in p {
                write!(self.output, "{} ", it);
            }
            writeln!(self.output, " {{");
            self.print_block_statement(b, indent);
            write!(self.output, "}}");

            self.print_deriving(d);
        }
    }

    fn print_deriving(&mut self, deriving: &Vec<DeriveType>) {
        if deriving.len() == 0 {
            writeln!(self.output, "");
        }
        else {
            write!(self.output, " deriving (");
            let mut first = true;
            for d in deriving {
                if !first {
                    write!(self.output, ", ");
                }
                write!(self.output, "{}", d);
                first = false;
            }
            // let d = deriving.clone();
            // write!(self.output, "{}", d.iter().join(", "));
            writeln!(self.output, ")");
        }
    }

    fn print_function_statement(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::Function(_, ref fm, ref i, ref p, ref dt) = *stmt_kind {
            self.print_spaces(indent);
            if *fm != FunctionModifier::None {
                write!(self.output, "{} ", fm);
            }
            write!(self.output, "{}(", i.value);
            let mut first = true;
            for param in p {
                if !first {
                    write!(self.output, ", ");
                }
                write!(self.output, "{}: {}", param.ident, param.data_type);
                first = false;
            }
            writeln!(self.output, "): {}", dt);
        }
    }

    fn print_comment_statement(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::Comment(_, ref s) = *stmt_kind {
            self.print_spaces(indent);
            writeln!(self.output, "#{}", s);
        }
    }

    fn print_import_statement(&mut self, stmt_kind: &StatementKind) {
        if let StatementKind::Import(_, ref s) = *stmt_kind {
            writeln!(self.output, "@import \"{}\"", s);
        }
    }

    fn print_const_statement(&mut self, stmt_kind: &StatementKind, mut indent: i32) {
        if let StatementKind::Const(_, ref i, ref dt, ref v) = *stmt_kind {
            self.print_spaces(indent);
            write!(self.output, "const {}: {} = ", i, dt);
            let kind = v.clone();
            self.print_statement_kind(&kind.stmtKind, indent);
            writeln!(self.output, ";");
        }
    }

    fn print_string_literal(&mut self, stmt_kind: &StatementKind, indent: i32) {
        if let StatementKind::StringLiteral(_, ref s) = *stmt_kind {
            write!(self.output, "\"{}\"", s);
        }
    }

    fn print_block(&mut self, stmt_kind: &StatementKind, mut indent: i32) {
        if let StatementKind::Block(ref b) = *stmt_kind {
            writeln!(self.output, " {{");
            indent = indent + 1;
            self.print_spaces(indent);
            write!(self.output, "{}", b);

            indent = indent - 1;
            writeln!(self.output, "");
            self.print_spaces(indent);
            write!(self.output, "}}");
        }
    }

    fn print_block_statement(&mut self, block: &BlockStatement, mut indent: i32) {
        indent = indent + 1;
        for s in &block.statements {
            self.print_statement(&s, indent);
        }
    }

    fn print_spaces(&mut self, indent: i32) {
        for _ in 0..(indent * 4) {
            write!(self.output, " ");
        }
    }

    fn print_newline(&mut self) {
        writeln!(self.output, "");
    }
}
