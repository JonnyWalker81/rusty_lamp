/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::collections::{ HashMap, HashSet };
use parser::ast::{Statement, StatementKind, DataTypeStatement};
use generator::resolver::ResolveError;

pub struct Typer {
    table: HashMap<String, Statement>,
    identifiers: HashSet<String>
}

impl Typer {
    pub fn new() -> Typer {
        let mut t = Typer {
            table: HashMap::new(),
            identifiers: HashSet::new()
        };

        t.populate_builtin_types();

        t
    }

    fn populate_builtin_types(&mut self) {
    }

    pub fn insert_type(&mut self, key: &String, stmt: &Statement) -> Result<(), ResolveError> {
        if self.identifiers.contains(key) {
            return Err(ResolveError::Duplicate(format!("Type already defined: {}", key)));
        }

        self.identifiers.insert(key.clone());
        self.table.insert(key.clone(), stmt.clone());
        return Ok(());
    }

    pub fn type_exists(&self, key: &String) -> bool {
        return self.identifiers.contains(key);
    }

    pub fn dup_check(&mut self, key: &String) -> Result<(), ResolveError>  {
        if self.identifiers.contains(key) {
            return Err(ResolveError::Duplicate(format!("Type already defined: {}", key)));
        }

        self.identifiers.insert(key.clone());
        Ok(())
    }
}
