// NONOS Operating System (AGPL-3.0-or-later)
//! History expansion, checked on the cases where getting it wrong runs
//! something the reader did not ask for.

mod designators;
mod plain;
mod refusals;

use crate::expand::{expand, Entries, ExpandError};

struct Hist(Vec<&'static str>);

impl Entries for Hist {
    fn count(&self) -> usize {
        self.0.len()
    }
    fn get(&self, i: usize) -> &[u8] {
        self.0[i].as_bytes()
    }
}

fn hist() -> Hist {
    Hist(vec!["ls -la", "cargo build", "grep foo src", "cargo test"])
}

fn run(line: &str) -> Option<Result<String, ExpandError>> {
    expand(line.as_bytes(), &hist()).map(|r| r.map(|v| String::from_utf8(v).unwrap()))
}
