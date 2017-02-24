/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::collections::{ HashMap, HashSet };
use parser::ast::{Statement, StatementKind, DataTypeStatement};
use generator::resolver::ResolveError;

pub struct Typer {
    table: HashMap<String, Statement>,
}

impl Typer {
    pub fn new() -> Typer {
        let mut t = Typer {
            table: HashMap::new(),
        };

        t.populate_builtin_types();

        t
    }

    fn populate_builtin_types(&mut self) {
    }

    pub fn insert_type(&mut self, key: &String, stmt: &Statement) -> Result<(), ResolveError> {
        self.table.insert(key.clone(), stmt.clone());
        return Ok(());
    }

    pub fn type_exists(&self, key: &String) -> bool {
        return self.table.contains_key(key);
    }
}

pub struct DuplicateChecker {
    identifiers: HashSet<String>,
    label: String
}

impl DuplicateChecker {
    pub fn new(label: String) -> DuplicateChecker {
        DuplicateChecker {
            identifiers: HashSet::new(),
            label: label
        }
    }

    pub fn check(&mut self, ident: &String) -> Result<(), ResolveError> {
        if self.identifiers.contains(ident) {
            return Err(ResolveError::Duplicate(self.label.clone(), format!("Type already defined: {}", ident.clone())));
        }

        self.identifiers.insert(ident.clone());
        Ok(())
    }
}
