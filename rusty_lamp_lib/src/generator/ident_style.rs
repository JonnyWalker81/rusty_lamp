/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

type IdentConverter = Fn(&String) -> String;

pub struct IdentStyle {
    
}

impl IdentStyle {
    pub fn firstUpper(token: &String) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_upper() {
        let token = String::from("fooBar");

        let result = IdentStyle::firstUpper(&token);
        println!("First Upper: {}", result);
    }
}
