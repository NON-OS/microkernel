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

//! Where a pattern starts in a line, as the search that found the line reads it.

use alloc::vec::Vec;

/// Every position in `hay` where `needle` starts, without overlapping.
///
/// Case folding matches the search that selected the line, or the marks would
/// land somewhere other than what was found.
pub(super) fn matches(hay: &[u8], needle: &[u8], fold: bool) -> Vec<usize> {
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
