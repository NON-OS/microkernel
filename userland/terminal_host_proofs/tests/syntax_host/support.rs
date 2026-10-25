// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Classifies a line into one part per byte.

use crate::syntax::{classify, Part};
use crate::term::theme::types::Theme;

/// A theme whose every role is its own colour, so a part's colour names its role.
pub const THEME: Theme = Theme { fg: 1, accent: 2, path: 3, dim: 4 };

pub fn parts_of(line: &str) -> Vec<Part> {
    let mut out = vec![Part::Plain; line.len()];
    classify(line.as_bytes(), &mut out);
    out
}
