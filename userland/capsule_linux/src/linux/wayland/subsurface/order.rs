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

//! The order a window's surfaces are painted in. Pure, so the host proofs
//! hold it.
//!
//! Each parent has a stack, bottom first, of its own surface and its
//! subsurfaces' (wl_subsurface place_above and place_below move a surface
//! within it). Painting a surface paints its stack: itself where it stands,
//! and each subsurface's stack in turn, where that one stands. A surface
//! that shows nothing hides everything placed on it, as Wayland has it.

use alloc::vec::Vec;

/// One parent's stack, bottom first: its subsurfaces' surfaces and its own.
pub struct Stack {
    pub parent: u32,
    pub ids: Vec<u32>,
}

/// The deepest a subsurface may be placed on another: deeper ones are not
/// painted, so a cycle the requests let through cannot run forever.
pub const MAX_DEPTH: usize = 16;

/// A surface to paint, and where its top-left corner is from the root's.
pub type Layer = (u32, (i32, i32));

/// The root's painting order: the surfaces beneath it, bottom first, and
/// the ones above it, bottom first. `at` gives a subsurface's position on
/// its parent, or None while it shows nothing.
pub fn layers(
    stacks: &[Stack],
    root: u32,
    at: &dyn Fn(u32) -> Option<(i32, i32)>,
) -> (Vec<Layer>, Vec<Layer>) {
    let mut all = Vec::new();
    paint(stacks, root, (0, 0), at, 0, &mut all);
    let split = all.iter().position(|(id, _)| *id == root).unwrap_or(0);
    let above = all.split_off(split + 1);
    all.truncate(split);
    (all, above)
}

fn paint(
    stacks: &[Stack],
    me: u32,
    (x, y): (i32, i32),
    at: &dyn Fn(u32) -> Option<(i32, i32)>,
    depth: usize,
    out: &mut Vec<Layer>,
) {
    let Some(stack) = stacks.iter().find(|s| s.parent == me) else {
        return out.push((me, (x, y)));
    };
    for &id in &stack.ids {
        if id == me {
            out.push((me, (x, y)));
        } else if let (Some((dx, dy)), true) = (at(id), depth < MAX_DEPTH) {
            paint(stacks, id, (x.saturating_add(dx), y.saturating_add(dy)), at, depth + 1, out);
        }
    }
}
