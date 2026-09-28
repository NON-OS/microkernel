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

/* Following a path, a name at a time, under a root. */

use alloc::vec::Vec;

use super::super::resolve::visible;
use super::super::synth::{self, Node};

/* The path with the first made link in it replaced by its target. */
pub(super) fn made_link(path: &[u8], last: bool) -> Option<Vec<u8>> {
    if !synth::owns(path) {
        return None;
    }
    let ends = path.iter().enumerate().skip(1).filter(|(_, b)| **b == b'/').map(|(i, _)| i);
    let whole = last.then_some(path.len());
    for end in ends.chain(whole) {
        let Some(Ok(Node::Link(to))) = synth::node(&path[..end]) else {
            continue;
        };
        /* A pipe or a socket: there is nothing to walk through. */
        if to.first() != Some(&b'/') && to.contains(&b':') {
            return None;
        }
        let dir = &path[..path[..end].iter().rposition(|b| *b == b'/').unwrap_or(0)];
        let mut joined = match to.first() == Some(&b'/') {
            true => Vec::new(),
            false => [dir, b"/"].concat(),
        };
        joined.extend_from_slice(&to);
        joined.extend_from_slice(&path[end..]);
        return Some(visible(b"/", &joined));
    }
    None
}
