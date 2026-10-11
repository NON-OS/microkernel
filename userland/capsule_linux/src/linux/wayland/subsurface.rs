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
//! and toolkits draw their decorations and overlays.
//!
//! A subsurface is made an object, so its id is the client's to use and to
//! destroy, and its surface's commits are answered like any other's. Its
//! pixels are not composed onto the window yet: only a surface with a
//! toplevel reaches the screen (commit.rs), so a subsurface never takes
//! the window's place.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;

pub fn get(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(id), Some(surface), Some(parent)) = (args.u32(), args.u32(), args.u32()) else {
        return;
    };
    let is_surface = |id| guest.objects.get(id) == Some(Object::Surface);
    if surface == parent || !is_surface(surface) || !is_surface(parent) {
        return;
    }
    let _ = guest.objects.put(id, Object::Subsurface);
}
