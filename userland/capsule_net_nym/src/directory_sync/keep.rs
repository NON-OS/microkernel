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

//! Cutting a fetched list to what the store will hold, out loud.
//!
//! The budgets cover the whole active set with headroom, so in normal
//! operation this drops nothing. If the network grows past one of them it
//! starts dropping again, and the previous version of that did it silently:
//! routes still built, traffic still flowed, and every client quietly shared
//! the same prefix of the list. A directory that is holding less than the
//! network published is a fact about anonymity, so it is said rather than
//! inferred later from a packet capture.

use alloc::vec::Vec;

use crate::topology::Node;

pub fn keep(mut found: Vec<Node>, budget: usize, role: &[u8]) -> Vec<Node> {
    if found.len() > budget {
        crate::trace::say(role);
        crate::trace::say_two(
            b"directory list exceeds its budget: published, kept",
            found.len() as u64,
            budget as u64,
        );
        found.truncate(budget);
    }
    found
}
