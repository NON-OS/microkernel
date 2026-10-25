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

//! The fields of each kind of line, checked to the byte.

use crate::redact_shapes::value_at;
use crate::redact_words::names_private;

pub(crate) fn hex(w: &[u8], n: usize) -> bool {
    w.len() == n && w.iter().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
}

/// A decimal with no leading zero, as the writer prints one.
pub(crate) fn dec(w: &[u8]) -> Option<u64> {
    if w.is_empty() || w.len() > 19 || (w.len() > 1 && w[0] == b'0') {
        return None;
    }
    w.iter().try_fold(0u64, |v, b| b.is_ascii_digit().then(|| v * 10 + u64::from(b - b'0')))
}

/// A line's fields, exactly four of them.
pub(crate) fn four(w: &[u8]) -> Option<[&[u8]; 4]> {
    let mut it = w.split(|b| *b == b' ');
    let f = [it.next()?, it.next()?, it.next()?, it.next()?];
    it.next().is_none().then_some(f)
}

fn word_ok(w: &[u8]) -> bool {
    let ok = |b: &u8| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_');
    !w.is_empty() && w.len() <= 40 && w.iter().all(ok)
}

pub(crate) fn build_ok(line: &[u8]) -> bool {
    let mut f = line.split(|b| *b == b' ');
    f.next() == Some(b"build")
        && f.next().is_some_and(word_ok)
        && f.next().is_some_and(word_ok)
        && f.next().is_none()
}

/// Printable ASCII, no private word, and no value the scrub would mask:
/// what the writer emits, and nothing that could have escaped it.
pub(crate) fn log_ok(w: &[u8]) -> bool {
    let clean = w.iter().all(|b| (0x20..0x7f).contains(b));
    clean && !names_private(w) && (0..w.len()).all(|i| value_at(w, i) == 0)
}

/// `count` and `cut` of an `end` line.
pub(crate) fn end_of(line: &[u8]) -> Option<(u64, bool)> {
    let rest = line.strip_prefix(b"end ")?;
    match rest.strip_suffix(b" cut") {
        Some(c) => Some((dec(c)?, true)),
        None => Some((dec(rest)?, false)),
    }
}
