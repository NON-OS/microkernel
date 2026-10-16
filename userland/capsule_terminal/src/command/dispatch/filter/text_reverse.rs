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

//! `tac` and `rev`: piped lines in reverse order, or each line reversed.

use alloc::vec::Vec;

/// Reverse the order of the lines. The counterpart to `tail` when what is
/// wanted is the whole thing, newest first.
pub fn tac(mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input.reverse();
    input
}

/// Reverse the bytes within each line, leaving the order of lines alone.
pub fn rev(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input
        .into_iter()
        .map(|mut line| {
            line.reverse();
            line
        })
        .collect()
}
