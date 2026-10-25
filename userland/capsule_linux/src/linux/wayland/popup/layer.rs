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

//! A grabbing popup as a layer of its window: a desynchronized subsurface
//! of the toplevel's surface, on top of its stack. So its commits show at
//! once, and it is composed, clipped to the window and held to the pixel
//! budget exactly as a subsurface is (subsurface/). A subsurface made
//! later is stacked below the popups (subsurface/get.rs).

use alloc::vec;

use super::super::scene::Scene;
use super::super::subsurface::order::Stack;
use super::super::subsurface::state::Sub;

/// Popup `id`'s `surface`, laid on top of the shown toplevel at `at`.
pub fn lay(scene: &mut Scene, id: u32, surface: u32, at: (i32, i32)) {
    let Some(root) = scene.shown.map(|s| s.surface) else { return };
    take(scene, surface);
    let subs = &mut scene.subs;
    let (next_at, shown, waiting) = (None, None, None);
    subs.list.push(Sub { id, surface, parent: root, at, next_at, sync: false, shown, waiting });
    match subs.stacks.iter_mut().find(|s| s.parent == root) {
        Some(stack) => stack.ids.push(surface),
        None => subs.stacks.push(Stack { parent: root, ids: vec![root, surface] }),
    }
}

/// A popup's `surface` taken off its window.
pub fn take(scene: &mut Scene, surface: u32) {
    let subs = &mut scene.subs;
    subs.list.retain(|s| s.surface != surface);
    for stack in subs.stacks.iter_mut() {
        stack.ids.retain(|i| *i != surface || *i == stack.parent);
    }
    subs.restack.retain(|r| r.0 != surface && r.1 != surface);
}
