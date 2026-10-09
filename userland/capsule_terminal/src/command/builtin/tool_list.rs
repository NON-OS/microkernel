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

//! The tools row of `help`, kept apart from the tool table that spawns them
//! so the proofs can build it.

use alloc::vec::Vec;

use super::help_layout::{GROUP_PAD, WIDTH};

/*
 * The tools, one name for each: two names for one service run the same
 * program, and listing both says there are two. A name that would push
 * the row past the width is left out rather than wrapped; the list points
 * at what exists.
 */
pub fn tool_list(tools: &[(&[u8], &[u8])]) -> Vec<u8> {
    let room = WIDTH - 2 - GROUP_PAD;
    let mut list: Vec<u8> = Vec::with_capacity(room);
    for (i, (typed, service)) in tools.iter().enumerate() {
        if tools[..i].iter().any(|(_, s)| s == service) {
            continue;
        }
        let gap = if list.is_empty() { 0 } else { 2 };
        if list.len() + gap + typed.len() > room {
            break;
        }
        list.resize(list.len() + gap, b' ');
        list.extend_from_slice(typed);
    }
    list
}
