/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

#![feature(conservative_impl_trait)]
use std::sync::Arc;
use std::collections::HashMap;

pub type IdentConverter = Fn(String) -> String;

// pub trait IdentConverter {
//     fn convert(input: &String) -> String;
// }
pub struct IdentStyleDefault {
    ty: Arc<IdentConverter>,
    enum_type: Arc<IdentConverter>,
    type_param: Arc<IdentConverter>,
    method: Arc<IdentConverter>,
    field: Arc<IdentConverter>,
    local: Arc<IdentConverter>,
    enm: Arc<IdentConverter>,
    cnst: Arc<IdentConverter>
}

impl IdentStyleDefault {
    pub fn new(ty: Arc<IdentConverter>, enum_type: Arc<IdentConverter>, type_param: Arc<IdentConverter>,
               method: Arc<IdentConverter>, field: Arc<IdentConverter>, local: Arc<IdentConverter>,
               enm: Arc<IdentConverter>, cnst: Arc<IdentConverter>) -> IdentStyleDefault {
        IdentStyleDefault {
            ty: ty,
            enum_type: enum_type,
            type_param: type_param,
            method: method,
            field: field,
            local: local,
            enm: enm,
            cnst: cnst
        }
    }
}

pub struct IdentStyle {
    pub cpp_style_default: IdentStyleDefault,
    pub java_style_default: IdentStyleDefault,
    pub objc_style_default: IdentStyleDefault,
    pub styles: HashMap<&'static str, Arc<IdentConverter>>
}



pub enum IdentStyleType {
    CppIdentStyle(Arc<IdentConverter>)
}

impl IdentStyle {
    pub fn new() -> IdentStyle {
        let mut styles_map: HashMap<&'static str, Arc<IdentConverter>> = HashMap::new();
        styles_map.insert("FooBar", Arc::new(IdentStyle::camel_upper));
        styles_map.insert("fooBar", Arc::new(IdentStyle::camel_lower));
        styles_map.insert("foo_bar", Arc::new(IdentStyle::under_lower));
        styles_map.insert("Foo_Bar", Arc::new(IdentStyle::under_upper));
        styles_map.insert("FOO_BAR", Arc::new(IdentStyle::under_caps));
        
        IdentStyle {
            cpp_style_default: IdentStyleDefault {
                ty: Arc::new(IdentStyle::camel_upper),
                enum_type: Arc::new(IdentStyle::camel_upper),
                type_param: Arc::new(IdentStyle::camel_upper),
                method: Arc::new(IdentStyle::camel_lower),
                field: Arc::new(IdentStyle::camel_lower),
                local: Arc::new(IdentStyle::camel_lower),
                enm: Arc::new(IdentStyle::under_caps),
                cnst: Arc::new(IdentStyle::under_caps),
            },
            java_style_default: IdentStyleDefault {
                ty: Arc::new(IdentStyle::camel_upper),
                enum_type: Arc::new(IdentStyle::camel_upper),
                type_param: Arc::new(IdentStyle::camel_upper),
                method: Arc::new(IdentStyle::camel_lower),
                field: Arc::new(IdentStyle::camel_lower),
                local: Arc::new(IdentStyle::camel_lower),
                enm: Arc::new(IdentStyle::under_caps),
                cnst: Arc::new(IdentStyle::under_caps),
            },
            objc_style_default: IdentStyleDefault {
                ty: Arc::new(IdentStyle::camel_upper),
                enum_type: Arc::new(IdentStyle::camel_upper),
                type_param: Arc::new(IdentStyle::camel_upper),
                method: Arc::new(IdentStyle::camel_lower),
                field: Arc::new(IdentStyle::camel_lower),
                local: Arc::new(IdentStyle::camel_lower),
                enm: Arc::new(IdentStyle::camel_upper),
                cnst: Arc::new(IdentStyle::camel_upper),
            },
            styles: styles_map
        }
    }

    pub fn first_upper(token: String) -> String {
        if token.len() == 0 {
            return token.clone();
        }
        else {
            let mut result = String::new();
            result = token.chars().nth(0).unwrap().to_uppercase().collect::<String>();
            result.push_str(token.chars().skip(1).collect::<String>().as_str());
            return result;
        }
    }

    pub fn camel_upper(s: String) -> String {
        let vec = s.split("_").map(|i| i.to_string()).collect::<Vec<String>>();
        let result = vec.into_iter().map(IdentStyle::first_upper).collect::<String>();
        return result;
    }

    pub fn camel_lower(s: String) -> String {
        let mut parts = s.split("_");
        let mut result = String::new();
        result.push_str(parts.nth(0).unwrap());
        result.push_str(parts.skip(1).map(|i| i.to_string()).into_iter().map(IdentStyle::first_upper).collect::<String>().as_str());
        return result;
    }

    pub fn under_lower(s: String) -> String {
        s
    }

    pub fn under_upper(s: String) -> String {
        let vec = s.split("_").map(|i| i.to_string()).collect::<Vec<String>>();
        let result = vec.into_iter().map(IdentStyle::first_upper).collect::<Vec<String>>().join("_");
        return result;
    }

    pub fn under_caps(s: String) -> String {
        s.to_uppercase()
    }

    pub fn prefix(pre: String, suffix: Arc<IdentConverter>) -> Arc<IdentConverter> {
        return Arc::new(move |s| {
            let mut result = String::new();
            result.push_str(pre.clone().as_str());
            result.push_str(suffix(s.clone()).as_str());
            result
        });
    }

    pub fn infer(styles: &HashMap<&'static str, Arc<IdentConverter>>, input: String) -> Option<Arc<IdentConverter>> {
        for (s, func) in styles {
            if(input.ends_with(s)) {
                let diff = input.len() - s.len();
                if (diff > 0) {
                    let before = input[0..diff].into();
                    return Some(IdentStyle::prefix(before, func.clone()))
                }
                else {
                    return Some(func.clone())
                }
            }
        }
        None
    }

    pub fn build_ident_style(name: &'static str) -> Arc<IdentConverter> {
        let ident_style = IdentStyle::new();

        let infer = IdentStyle::infer(&ident_style.styles, name.into());
        match infer {
            Some(i) => {
                i
            },
            None => {
                panic!("invalid ident spec: {}", name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_upper() {
        let token = String::from("fooBar");

        let result = IdentStyle::first_upper(token);
        println!("First Upper: {}", result);
    }

    #[test]
    fn test_infer() {
        let ident_style = IdentStyle::new();

        let infer = IdentStyle::infer(&ident_style.styles, "foo_bar".into());
        match infer {
            Some(i) => {
                println!("{}", i("my_record".into()));
            },
            None => {
                println!("None...");
            }
        }
    }
}
