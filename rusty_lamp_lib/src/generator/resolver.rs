/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::ast::{BlockStatement, Statement, StatementKind};
use parser::program::Program;
use generator::typer::{ Typer, DuplicateChecker, TypeDefinitionKind };

pub struct Resolver {
    typer: Typer
}

#[derive(Debug)]
pub enum ResolveError {
    Resolve(String),
    Duplicate(String, String),
    ExpectedEnumOption,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            typer: Typer::new()
        }
    }

    pub fn resolve(&mut self, program: &Program) -> Result<(), ResolveError> {
        let mut dup_checker = DuplicateChecker::new("Top Level".into());
        for stmt in &program.statements {
            self.resolve_statement(&stmt, &mut dup_checker)?
        }

        self.typer.dump();
        return Ok(());
    }

    fn resolve_statement(&mut self, stmt: &Statement, dup_checker: &mut DuplicateChecker) -> Result<(), ResolveError> {
        match stmt.stmtKind {
            StatementKind::Enum(_, ref i, _) => {
                dup_checker.check(&i.value)?;
                self.resolve_enum(&stmt)?
            },
            StatementKind::Record(_, ref i, _, _) => {
                dup_checker.check(&i.value)?;
                self.resolve_record(&stmt)?
            },
            StatementKind::Interface(_, ref i, _, _, _) => {
            dup_checker.check(&i.value)?;
                self.resolve_interface(&stmt)?
            }
            _ => {
                
            }
        }

        Ok(())
    }

    fn resolve_enum(&mut self, stmt: &Statement) -> Result<(), ResolveError> {
        if let StatementKind::Enum(_, ref id, ref b) = stmt.stmtKind {
            let mut dup_checker = DuplicateChecker::new("Enum".into());
            self.typer.insert_type(&id.value, TypeDefinitionKind::UserObject(id.value.clone()))?;
            for s in &b.statements {
                match s.stmtKind {
                    StatementKind::EnumMember(_, ref i) => {
                        dup_checker.check(&i.value)?
                    },
                    _ => {
                        // return Err(ResolveError::ExpectedEnumOption);
                    }
                }
            }
        }

        Ok(())
    }

    fn resolve_record(&mut self, stmt: &Statement) -> Result<(), ResolveError> {
        if let StatementKind::Record(_, ref id, ref b, ref dt) = stmt.stmtKind {
            let mut dup_checker = DuplicateChecker::new("Record".into());
            self.typer.insert_type(&id.value, TypeDefinitionKind::UserObject(id.value.clone()))?;
            for s in &b.statements {
                match s.stmtKind {
                    StatementKind::RecordMember(_, ref i, ref dts) => {
                        dup_checker.check(&i.value)?;
                    },
                    _ => {
                        // return Err(ResolveError::ExpectedEnumOption);
                    }
                }
            }
        }

        Ok(())
    }

    fn resolve_interface(&mut self, stmt: &Statement) -> Result<(), ResolveError> {
        if let StatementKind::Interface(_, ref id, ref it, ref b, ref dt) = stmt.stmtKind {
            let mut dup_checker = DuplicateChecker::new("Interface".into());
            self.typer.insert_type(&id.value, TypeDefinitionKind::UserObject(id.value.clone()))?;
            for s in &b.statements {
                match s.stmtKind {
                    StatementKind::Function(_, ref fm, ref i, ref p, ref dts) => {
                        dup_checker.check(&i.value)?;
                    },
                    _ => {
                        // return Err(ResolveError::ExpectedEnumOption);
                    }
                }
            }
        }

        Ok(())
    }
}
