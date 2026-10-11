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

//! `wl_subcompositor.get_subsurface`: a surface placed on another, as foot
//! and toolkits draw their overlays. It starts synchronized, at its
//! parent's corner, on top of its parent's stack, and shows nothing until
//! it commits a buffer. A surface with a role already, or a parent placed
//! on the surface itself, gets no subsurface.

use alloc::vec;

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::order::Stack;
use super::restack::placed_on;
use super::state::Sub;

pub fn get(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(id), Some(surface), Some(parent)) = (args.u32(), args.u32(), args.u32()) else {
        return;
    };
    let is_surface = |id| guest.objects.get(id) == Some(Object::Surface);
    if surface == parent || !is_surface(surface) || !is_surface(parent) {
        return;
    }
    let scene = &guest.scene;
    let shell = scene.surfaces.iter().any(|s| s.id == surface && s.xdg.is_some());
    if shell || scene.subs.of(surface).is_some() || placed_on(&scene.subs, parent, surface) {
        return;
    }
    if !guest.objects.put(id, Object::Subsurface) {
        return;
    }
    let subs = &mut guest.scene.subs;
    subs.list.push(Sub {
        id,
        surface,
        parent,
        at: (0, 0),
        next_at: None,
        sync: true,
        shown: None,
        waiting: None,
    });
    match subs.stacks.iter_mut().find(|s| s.parent == parent) {
        Some(stack) => stack.ids.push(surface),
        None => subs.stacks.push(Stack { parent, ids: vec![parent, surface] }),
    }
}
