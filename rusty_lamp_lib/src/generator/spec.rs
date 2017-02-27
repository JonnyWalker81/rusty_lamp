/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use generator::typer::Typer;

pub struct Spec {
    pub root: String,
    pub cpp_root: String,
    pub typer: Typer
}

impl Spec {
    pub fn new(root: String, cpp_root: String, typer: Typer) -> Spec {
        Spec{
            root: root,
            cpp_root: cpp_root,
            typer: typer
        }
    }
}
