/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::ast::{BlockStatement, Statement, StatementKind};
use parser::program::Program;
use generator::typer::Typer;

pub struct Resolver {
    typer: Typer
}

#[derive(Debug)]
pub enum ResolveError {
    Resolve(String),
    Duplicate(String),
    ExpectedEnumOption,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            typer: Typer::new()
        }
    }

    pub fn resolve(&mut self, program: &Program) -> Result<(), ResolveError> {
        for stmt in &program.statements {
            self.resolve_statement(&stmt)?
        }
        return Ok(());
    }

    fn resolve_statement(&mut self, stmt: &Statement) -> Result<(), ResolveError> {
        match stmt.stmtKind {
            StatementKind::Enum(..) => {
                self.resolve_enum(&stmt)?
            },
            StatementKind::Record(..) => {
                self.resolve_record(&stmt)?
            }
            _ => {
                
            }
        }

        Ok(())
    }

    fn resolve_enum(&mut self, stmt: &Statement) -> Result<(), ResolveError> {
        if let StatementKind::Enum(_, _, ref b) = stmt.stmtKind {
            let mut typer = Typer::new();
            for s in &b.statements {
                match s.stmtKind {
                    StatementKind::EnumMember(_, ref i) => {
                        typer.dup_check(&i.value)?
                    },
                    _ => {
                        return Err(ResolveError::ExpectedEnumOption);
                    }
                }
            }
        }

        Ok(())
    }

    fn resolve_record(&mut self, stmt: &Statement) -> Result<(), ResolveError> {
        if let StatementKind::Record(_, _, ref b, ref dt) = stmt.stmtKind {
            let mut typer = Typer::new();
            for s in &b.statements {
                match s.stmtKind {
                    StatementKind::RecordMember(_, ref i, ref dts) => {
                        typer.dup_check(&i.value)?
                    },
                    _ => {
                        return Err(ResolveError::ExpectedEnumOption);
                    }
                }
            }
        }

        Ok(())
    }
}
