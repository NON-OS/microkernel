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

//! What a commit on a parent applies: its subsurfaces' new positions, the
//! synchronized ones' waiting commits, and their new places in its stack.
//! Nesting is taken one level at a time: a commit applies what its own
//! subsurfaces left waiting, whether or not its own commit waits.

use super::order::MAX_DEPTH;
use super::state::Subs;

/// A commit on `parent`.
pub fn parent_commit(subs: &mut Subs, parent: u32) {
    for sub in subs.list.iter_mut().filter(|s| s.parent == parent) {
        sub.at = sub.next_at.take().unwrap_or(sub.at);
        if let (true, Some(waiting)) = (sub.sync, sub.waiting.take()) {
            sub.shown = waiting;
        }
    }
    let Some(stack) = subs.stacks.iter_mut().find(|s| s.parent == parent) else { return };
    let mut rest = alloc::vec::Vec::new();
    for (surface, reference, above) in subs.restack.drain(..) {
        if !stack.ids.contains(&surface) {
            rest.push((surface, reference, above));
            continue;
        }
        stack.ids.retain(|id| *id != surface);
        // A reference gone since the request leaves the surface on top.
        let at = match stack.ids.iter().position(|id| *id == reference) {
            Some(i) if above => i + 1,
            Some(i) => i,
            None => stack.ids.len(),
        };
        stack.ids.insert(at, surface);
    }
    subs.restack = rest;
}

/// Whether `parent` is `surface` or is placed on it, however deep.
pub fn placed_on(subs: &Subs, mut parent: u32, surface: u32) -> bool {
    for _ in 0..=MAX_DEPTH {
        if parent == surface {
            return true;
        }
        match subs.of(parent) {
            Some(s) => parent = s.parent,
            None => return false,
        }
    }
    true
}
