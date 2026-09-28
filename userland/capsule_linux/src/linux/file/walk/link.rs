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

/* The link at a name, if a walk can go through it. */

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::synth::{self, Node};

/* Where the link at `at` leads, if `at` is one that can be walked through. */
pub(super) fn link_at(guest: &Guest, at: &[u8]) -> Option<Vec<u8>> {
    if let Some(to) = guest.links.target(at) {
        return Some(to);
    }
    match synth::node(at)? {
        /* A pipe or a socket: there is nothing to walk through. */
        Ok(Node::Link(to)) if to.first() == Some(&b'/') || !to.contains(&b':') => Some(to),
        _ => None,
    }
}
