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

//! Marking what grep matched, inside the line it matched in.
//!
//! A result list where every row looks the same makes the reader find the
//! match again by eye, on every row, which is the work they ran grep to avoid.
//! Marking it is the difference between a list to read and an answer.
//!
//! The styled form is built beside the plain one rather than derived from it,
//! because the scrollback sends `plain` to a pipe or a file and `styled` only
//! to the screen. Colour never reaches a capture, so `grep x > out` holds text
//! and not escapes.

use alloc::vec::Vec;

/// Every position in `hay` where `needle` starts, without overlapping.
///
/// Case folding matches the search that selected the line, or the marks would
/// land somewhere other than what was found.
fn matches(hay: &[u8], needle: &[u8], fold: bool) -> Vec<usize> {
    let mut at = Vec::new();
    if needle.is_empty() || needle.len() > hay.len() {
        return at;
    }
    let eq = |a: u8, b: u8| if fold { a.eq_ignore_ascii_case(&b) } else { a == b };
    let mut i = 0;
    while i + needle.len() <= hay.len() {
        if hay[i..].iter().zip(needle).all(|(h, n)| eq(*h, *n)) {
            at.push(i);
            i += needle.len();
        } else {
            i += 1;
        }
    }
    at
}

/// The line with its matches wrapped in a highlight, or `None` when there is
/// nothing to mark and the plain row will do.
///
/// `prefix_len` is how much of `row` is the file name and line number, which
/// is skipped so a pattern that happens to occur in a path is not marked as
/// though it were a hit.
pub(super) fn highlight(
    row: &[u8],
    prefix_len: usize,
    needle: &[u8],
    fold: bool,
) -> Option<Vec<u8>> {
    const ON: &[u8] = b"\x1b[1;33m";
    const OFF: &[u8] = b"\x1b[0m";

    let body = row.get(prefix_len..)?;
    let at = matches(body, needle, fold);
    if at.is_empty() {
        return None;
    }
    let mut out = Vec::with_capacity(row.len() + at.len() * (ON.len() + OFF.len()));
    out.extend_from_slice(&row[..prefix_len]);
    let mut cut = 0;
    for start in at {
        out.extend_from_slice(&body[cut..start]);
        out.extend_from_slice(ON);
        out.extend_from_slice(&body[start..start + needle.len()]);
        out.extend_from_slice(OFF);
        cut = start + needle.len();
    }
    out.extend_from_slice(&body[cut..]);
    Some(out)
}
