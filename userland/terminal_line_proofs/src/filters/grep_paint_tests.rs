// NONOS Operating System (AGPL-3.0-or-later)
//! Marking grep's matches inside the line, and the cases where marking would
//! be wrong.

mod edges;
mod marks;

use crate::grep_paint::highlight;

const ON: &str = "\x1b[1;33m";
const OFF: &str = "\x1b[0m";

fn hl(row: &str, prefix: usize, needle: &str, fold: bool) -> Option<String> {
    highlight(row.as_bytes(), prefix, needle.as_bytes(), fold)
        .map(|v| String::from_utf8(v).unwrap())
}
