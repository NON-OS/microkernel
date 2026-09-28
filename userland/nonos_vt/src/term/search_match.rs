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

//! Where a needle starts in a logical line's characters.

use alloc::vec::Vec;

use super::types::Pos;

fn eq(a: char, b: char, case: bool) -> bool {
    if case {
        a == b
    } else {
        a.to_lowercase().eq(b.to_lowercase())
    }
}

/// Every place `needle` starts in `hay`, by index.
pub(super) fn matches(hay: &[(char, Pos)], needle: &[char], case: bool) -> Vec<usize> {
    if needle.is_empty() || needle.len() > hay.len() {
        return Vec::new();
    }
    (0..=hay.len() - needle.len())
        .filter(|&i| needle.iter().enumerate().all(|(k, &c)| eq(hay[i + k].0, c, case)))
        .collect()
}
