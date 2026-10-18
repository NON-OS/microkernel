// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! `!!`, `!n` and `!prefix`: running something again without retyping it.
//!
//! Ctrl-R finds an old command and Up walks back through them, and both put
//! the reader in the history to pick one. Expansion is for when they already
//! know: the command just run, or the one that began with `cargo`. It is the
//! shortest path from "that again" to it happening.
//!
//! Deliberately narrow. Only a token that is entirely a `!` form expands, so a
//! `!` inside a word or a pattern is left alone and nothing a reader typed for
//! another purpose is rewritten underneath them.

use alloc::vec::Vec;

#[path = "expand_designator.rs"]
mod expand_designator;
use expand_designator::{designator, lookup};

/// What went wrong, so the caller can say so rather than run something else.
#[derive(Debug, PartialEq, Eq)]
pub enum ExpandError {
    /// A `!` form named an entry that is not there.
    NoMatch,
}

/// Look up one history entry. `count` is how many there are; `get(i)` reads
/// the i-th, oldest first, matching the History type this serves.
pub trait Entries {
    fn count(&self) -> usize;
    fn get(&self, index: usize) -> &[u8];
}

/// Rewrite any `!` tokens in `line` against `history`.
///
/// Returns `None` when there was nothing to expand, so the caller can tell a
/// rewritten line from an untouched one and echo only the former: seeing what
/// `!!` turned into is most of why it is safe to use.
pub fn expand<E: Entries>(line: &[u8], history: &E) -> Option<Result<Vec<u8>, ExpandError>> {
    if !line.contains(&b'!') {
        return None;
    }
    let mut out = Vec::with_capacity(line.len());
    let mut changed = false;
    for (i, token) in line.split(|&b| b == b' ').enumerate() {
        if i > 0 {
            out.push(b' ');
        }
        match designator(token) {
            None => out.extend_from_slice(token),
            Some(d) => match lookup(d, history) {
                Some(found) => {
                    out.extend_from_slice(found);
                    changed = true;
                }
                None => return Some(Err(ExpandError::NoMatch)),
            },
        }
    }
    changed.then_some(Ok(out))
}
