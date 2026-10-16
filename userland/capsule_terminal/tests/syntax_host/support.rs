// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Classifies a line into one part per byte.

use crate::syntax::{classify, Part};

pub fn parts_of(line: &str) -> Vec<Part> {
    let mut out = vec![Part::Plain; line.len()];
    classify(line.as_bytes(), &mut out);
    out
}
