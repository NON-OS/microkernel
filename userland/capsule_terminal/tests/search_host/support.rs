// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Builds a history from lines, oldest first.

use crate::types::History;

pub fn history(lines: &[&str]) -> History {
    let mut h = History::new();
    for line in lines {
        h.push(line.as_bytes());
    }
    h
}
