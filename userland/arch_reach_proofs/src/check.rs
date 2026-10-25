// NONOS Operating System (AGPL-3.0-or-later)
//! A ledger of counts set against the tree. The two must agree exactly: a
//! file the ledger misses or undercounts is x86_64-only code nobody has
//! explained; one it overcounts or names needlessly is a row gone stale,
//! which would let the next x86_64-only line in unexplained.

use std::collections::BTreeMap;
use std::fmt;

use crate::ledger::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// The value is not a count of one or more.
    BadCount { path: String },
    /// The tree names it `n` times and the ledger not at all.
    Unlisted { path: String, n: usize },
    /// The tree names it more often than its row says.
    Grown { path: String, row: usize, n: usize },
    /// The tree names it less often than its row says.
    Shrunk { path: String, row: usize, n: usize },
    /// The row's file names it no longer, or is gone.
    Gone { path: String },
}

pub fn compare(rows: &[Row], tree: &BTreeMap<String, usize>) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in rows {
        let path = row.key.clone();
        let Some(want) = row.value.parse::<usize>().ok().filter(|n| *n > 0) else {
            out.push(Finding::BadCount { path });
            continue;
        };
        match tree.get(&row.key).copied() {
            None => out.push(Finding::Gone { path }),
            Some(n) if n > want => out.push(Finding::Grown { path, row: want, n }),
            Some(n) if n < want => out.push(Finding::Shrunk { path, row: want, n }),
            Some(_) => {}
        }
    }
    for (path, n) in tree {
        if !rows.iter().any(|r| &r.key == path) {
            out.push(Finding::Unlisted { path: path.clone(), n: *n });
        }
    }
    out
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadCount { path } => write!(f, "{path}: the row's count is not a number above 0"),
            Self::Unlisted { path, n } => {
                write!(f, "{path}: {n} x86_64-only lines and no row saying why")
            }
            Self::Grown { path, row, n } => {
                write!(f, "{path}: {n} lines, the row says {row}: explain the new ones")
            }
            Self::Shrunk { path, row, n } => write!(f, "{path}: {n} lines, lower the row's {row}"),
            Self::Gone { path } => write!(f, "{path}: names it no more, drop the row"),
        }
    }
}
