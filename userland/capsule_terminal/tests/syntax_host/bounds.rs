// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! The output buffer bound.

use crate::syntax::{classify, Part};

/// The caller draws by byte, so every byte has to be classified even when the
/// buffer given is shorter than the line.
#[test]
fn a_short_buffer_is_not_written_past() {
    let line = "ls -la /etc";
    let mut out = vec![Part::Plain; 4];
    classify(line.as_bytes(), &mut out);
    assert_eq!(out.len(), 4);
    assert_eq!(out[0], Part::Command);
}
