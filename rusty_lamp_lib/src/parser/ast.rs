/*
 * Copyright © 2002-2017 Bluebeam Software, Inc. All Rights Reserved.
 * Creator: Jonathan Rothberg
 */

use parser::token::{Token, DataType}
use std::fmt;

pub struct Statement {
    pub stmtKind: StatementKind
}

pub enum StatementKind {
    Import(Token, String)
}
