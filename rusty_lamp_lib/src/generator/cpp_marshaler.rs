/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::ast::{DataTypeStatement};
use generator::typer::{TypeDefinitionKind};

pub struct CppMarshaler {
    
}

impl CppMarshaler {
    pub fn new() -> CppMarshaler {
        CppMarshaler {
            
        }
    }

    pub fn include(&self, dts: &DataTypeStatement) -> String {
        let include_file = match *dts {
            DataTypeStatement::Map(..) => {
                "<unordered_map>".into()
            },
            DataTypeStatement::Set(..) => {
                "<unordered_set>".into()
            },
            DataTypeStatement::List(..) => {
                "<vector>".into()
            },
            DataTypeStatement::String => {
                "<string>".into()
            },
            _ => {"".into()}
        };

        include_file
    }

    pub fn get_type_name(&self, t: TypeDefinitionKind) -> String {
        match t {
            TypeDefinitionKind::Map => "std::unordered_map".into(),
            TypeDefinitionKind::Set => "std::unordered_set".into(),
            TypeDefinitionKind::List => "std::vector".into(),
            TypeDefinitionKind::String => "std::string".into(),
            TypeDefinitionKind::Primitive(_, _, _, ref ct, _, _, _, _) => ct.clone(),
            TypeDefinitionKind::UserObject(ref n) => n.clone(),
            _ => "".into()
        }
    }
}
