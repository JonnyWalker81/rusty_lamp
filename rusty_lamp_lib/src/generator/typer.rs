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
    None,
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
            TypeDefinitionKind::None => "none".into(),
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
    pub fn num_params(&self) -> i8 {
        match *self {
            TypeDefinitionKind::Map => 2,
            TypeDefinitionKind::List | TypeDefinitionKind::Set | TypeDefinitionKind::Optional => 1,
            _ => 0
        }
    }

    // pub fn get_type_definition(&self, dts: &DataTypeStatement) -> &TypeDefinitionKind {
    //     match *dts {
    //         DataTypeStatement::Map(..) =>,
    //         DataTypeStatement::Set(..) => TypeDefinitionKind::Set,
    //         DataTypeStatement::List(..) => TypeDefinitionKind::List,
    //         DataTypeStatement::
    //     }
    // }
}

#[derive(Clone)]
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
        self.table.insert("i8".into(), TypeDefinitionKind::Primitive("i8".into(), "byte".into(), "jbyte".into(), "int8_t".into(), "Byte".into(), "B".into(), "int8_t".into(), "NSNumber".into()));
        self.table.insert("i16".into(), TypeDefinitionKind::Primitive("i16".into(), "short".into(), "short".into(), "int16_6".into(), "Short".into(), "S".into(), "int16_t".into(), "NSNumber".into()));
        self.table.insert("i32".into(), TypeDefinitionKind::Primitive("i32".into(), "int".into(), "jint".into(), "int32_t".into(), "Integer".into(), "I".into(), "int32_t".into(), "NSNumber".into()));
        self.table.insert("i64".into(), TypeDefinitionKind::Primitive("i64".into(), "long".into(), "jlong".into(), "int64_t".into(), "Long".into(), "J".into(), "int64_t".into(), "NSNumber".into()));
        self.table.insert("f32".into(), TypeDefinitionKind::Primitive("f32".into(), "float".into(), "jfloat".into(), "float".into(), "Float".into(), "F".into(), "float".into(), "NSNumber".into()));
        self.table.insert("f64".into(), TypeDefinitionKind::Primitive("f64".into(), "double".into(), "jdouble".into(), "double".into(), "Double".into(), "D".into(), "double".into(), "NSNumber".into()));
        self.table.insert("bool".into(), TypeDefinitionKind::Primitive("bool".into(), "boolean".into(), "jboolean".into(), "bool".into(), "Boolean".into(), "Z".into(), "BOOL".into(), "NSNumber".into()));
        self.table.insert("string".into(), TypeDefinitionKind::String);
        self.table.insert("binary".into(), TypeDefinitionKind::Binary);
        self.table.insert("optional".into(), TypeDefinitionKind::Optional);
        self.table.insert("date".into(), TypeDefinitionKind::Date);
        self.table.insert("list".into(), TypeDefinitionKind::List);
        self.table.insert("set".into(), TypeDefinitionKind::Set);
        self.table.insert("map".into(), TypeDefinitionKind::Map);
    }

    pub fn insert_type(&mut self, key: &String, td: TypeDefinitionKind) -> Result<(), ResolveError> {
        self.table.insert(key.clone(), td.clone());
        return Ok(());
    }

    pub fn get(&self, key: &String) -> TypeDefinitionKind {
        match self.table.get(key) {
            Some(e) => e.clone(),
            None => TypeDefinitionKind::None
        }
    }

    pub fn get_from_data_type(&self, dts: &DataTypeStatement) -> TypeDefinitionKind {
        match self.table.get(&dts.get_name()) {
            Some(t) => t.clone(),
            None => TypeDefinitionKind::None
        }
    }

    pub fn dump(&self) {
        for (key, value) in &self.table {
            println!("{} -> {}", key, value);
        }
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
