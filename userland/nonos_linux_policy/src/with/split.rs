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

//! Where a tail's listed arguments end and its extras begin, and the
//! package rule each extra is held to.

use crate::args;

/// A tail split into its arguments and its extras, each still to be checked
/// whole: a tail that opens with the extras has no arguments; any other is
/// arguments up to where their own lengths end.
pub fn split(tail: &[u8]) -> (&[u8], &[u8]) {
    if tail.starts_with(super::MAGIC) {
        return (&[], tail);
    }
    let Some(&words) = tail.get(args::MAGIC.len()) else { return (tail, &[]) };
    let walk = |at: usize, _| at + 1 + usize::from(tail.get(at).copied().unwrap_or(0));
    tail.split_at((0..words).fold(args::MAGIC.len() + 1, walk).min(tail.len()))
}

/// The catalogue's package-name rule (names.py, PACKAGE).
pub(super) fn package(name: &[u8]) -> bool {
    let rest = |c: &u8| c.is_ascii_lowercase() || c.is_ascii_digit() || b"._+-".contains(c);
    let first = name.first().is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    first && name.len() <= 64 && name.iter().all(rest)
}
