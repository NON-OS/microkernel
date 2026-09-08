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

enum Designator<'a> {
    /// `!!`, the previous command.
    Last,
    /// `!n`, counting from one as `history` prints them.
    Index(usize),
    /// `!text`, the most recent command starting with `text`.
    Prefix(&'a [u8]),
}

/// Read a whole token as a designator, or nothing.
///
/// A token must be entirely the form for it to count. `foo!bar` and a bare `!`
/// are ordinary text, which keeps this away from anything a reader meant
/// literally.
fn designator(token: &[u8]) -> Option<Designator<'_>> {
    let rest = token.strip_prefix(b"!")?;
    if rest.is_empty() {
        return None;
    }
    if rest == b"!" {
        return Some(Designator::Last);
    }
    if rest.iter().all(|b| b.is_ascii_digit()) {
        // Saturating, not checked. A number too large to hold is still
        // unmistakably an index, and returning "not a designator" here would
        // hand `!99999999999999999999999999` back as literal text for the
        // shell to try to run. Saturated, it is an index past the end and is
        // refused, which is what the reader meant to be told.
        let mut n = 0usize;
        for b in rest {
            n = n.saturating_mul(10).saturating_add((b - b'0') as usize);
        }
        return Some(Designator::Index(n));
    }
    Some(Designator::Prefix(rest))
}

fn lookup<'e, E: Entries>(d: Designator<'_>, history: &'e E) -> Option<&'e [u8]> {
    let count = history.count();
    match d {
        Designator::Last => (count > 0).then(|| history.get(count - 1)),
        // One-based, because that is how `history` numbers what it prints and
        // an off-by-one here runs a command the reader did not choose.
        Designator::Index(n) => (n >= 1 && n <= count).then(|| history.get(n - 1)),
        Designator::Prefix(p) => {
            (0..count).rev().map(|i| history.get(i)).find(|e| e.starts_with(p))
        }
    }
}
