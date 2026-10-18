// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Parses a byte-string argument list.

use crate::ls_flags::{parse, LsFlags};

pub fn run<'a>(args: &[&'a [u8]]) -> (LsFlags, Vec<&'a [u8]>) {
    parse(args).expect("expected the arguments to parse")
}
