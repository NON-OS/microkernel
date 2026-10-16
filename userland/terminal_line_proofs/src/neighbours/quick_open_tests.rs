// NONOS Operating System (AGPL-3.0-or-later)
//! Quick open: finding a file from a few letters. The ranking is the feature.
//! A matcher that finds the right file second is one nobody trusts.

mod matching;
mod ranking;

use crate::quick_open::best;

const TREE: &[&[u8]] = &[
    b"/README",
    b"/src/main.rs",
    b"/src/editor/mod.rs",
    b"/src/editor/paint.rs",
    b"/docs/main/notes.md",
];

fn find(q: &str) -> Option<&'static str> {
    best(TREE, q.as_bytes()).map(|i| core::str::from_utf8(TREE[i]).unwrap())
}
