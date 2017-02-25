/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use std::collections::{ HashMap, HashSet };
use std::fmt;
use parser::ast::{Statement, StatementKind, DataTypeStatement};
use generator::resolver::ResolveError;

pub struct TypeDefinition {
    identifier: String,
    num_params: i8,
    
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum TypeDefinitionKind {
    Primitive(String, String, String, String, String, String, String, String),
    String,
    Binary,
    Optional,
    Date,
    List,
    Set,
    Map,
    UserObject(String)
}

impl fmt::Display for TypeDefinitionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TypeDefinitionKind::Primitive(ref n, ..) => {
                format!("{}", n)
            },
            TypeDefinitionKind::String => {
                "string".into()
            },
            TypeDefinitionKind::Binary => {
                "binary".into()
            },
            TypeDefinitionKind::Optional => {
                "optional".into()
            },
            TypeDefinitionKind::Date => {
                "date".into()
            },
            TypeDefinitionKind::List => {
                "list".into()
            },
            TypeDefinitionKind::Set => {
                "set".into()
            },
            TypeDefinitionKind::Map => {
                "map".into()
            },
            TypeDefinitionKind::UserObject(ref n) => {
                format!("{}", n)
            }
        };

        write!(f, "{}", printable)
    }

    
}

impl TypeDefinitionKind {
    fn num_params(&self) -> i8 {
        match *self {
            TypeDefinitionKind::Map => 2,
            TypeDefinitionKind::List | TypeDefinitionKind::Set | TypeDefinitionKind::Optional => 1,
            _ => 0
        }
    }
}

pub struct Typer {
    table: HashMap<String, TypeDefinitionKind>,
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
        // self.table.insert("i8", TypeDefinitionKind::Primitive("i8".into(), ));
    }

    pub fn insert_type(&mut self, key: &String, td: &TypeDefinitionKind) -> Result<(), ResolveError> {
        self.table.insert(key.clone(), td.clone());
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
