// NONOS Operating System (AGPL-3.0-or-later)
//! "did you mean". A suggester that proposes nonsense is worse than silence,
//! so the refusals matter more here than the hits.

mod distance;
mod hits;
mod refusals;
mod ties;

use crate::suggest::nearest_two;

const NAMES: &[&[u8]] = &[
    b"ls",
    b"cat",
    b"cd",
    b"grep",
    b"help",
    b"history",
    b"tree",
    b"type",
    b"which",
    b"capsules",
    b"clear",
    b"echo",
    b"exit",
    b"mkdir",
    b"touch",
    b"tokei",
    b"rg",
];

fn near(typed: &str) -> Option<String> {
    nearest_two(typed.as_bytes(), NAMES.iter().copied())
        .0
        .map(|c| String::from_utf8(c.to_vec()).unwrap())
}
