// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Quoted text, closed and unclosed.

use crate::support::parts_of;
use crate::syntax::Part;

#[test]
fn quoted_text_is_one_part_including_its_spaces() {
    let line = "echo \"a b\"";
    let p = parts_of(line);
    let open = line.find('"').unwrap();
    for i in open..line.len() {
        assert_eq!(p[i], Part::Quoted, "byte {i}");
    }
}

/// An unclosed quote runs to the end rather than falling back mid line, which
/// is what shows the reader the quote is still open.
#[test]
fn an_unclosed_quote_runs_to_the_end() {
    let line = "echo \"still open";
    let p = parts_of(line);
    assert_eq!(p[line.len() - 1], Part::Quoted);
}
