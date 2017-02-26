/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

pub struct Spec {
    pub root: String,
    pub cpp_root: String
}

impl Spec {
    pub fn new(root: String, cpp_root: String) -> Spec {
        Spec{
            root: root,
            cpp_root: cpp_root
        }
    }
}
