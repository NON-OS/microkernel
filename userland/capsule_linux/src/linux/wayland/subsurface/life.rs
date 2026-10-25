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

//! A subsurface taken away: its wl_subsurface destroyed, which leaves its
//! surface and whatever is placed on that; or its surface destroyed, which
//! takes down too what was placed on it. The window is painted without it.

use crate::linux::guest::Guest;

use super::super::object::Object;
use super::compose::repaint;

/// `id`, a wl_subsurface or a wl_surface (`what`), is being destroyed.
pub fn gone(guest: &mut Guest, what: Option<Object>, id: u32) {
    let subs = &mut guest.scene.subs;
    let before = subs.list.len();
    let surface = match what {
        Some(Object::Subsurface) => match subs.list.iter().find(|s| s.id == id) {
            Some(s) => s.surface,
            None => return,
        },
        Some(Object::Surface) => {
            // Its own stack goes, and what was placed on it is inert.
            subs.stacks.retain(|s| s.parent != id);
            subs.list.retain(|s| s.parent != id);
            id
        }
        _ => return,
    };
    subs.list.retain(|s| s.surface != surface);
    for stack in subs.stacks.iter_mut() {
        stack.ids.retain(|i| *i != surface || *i == stack.parent);
    }
    subs.restack.retain(|r| r.0 != surface && r.1 != surface);
    if subs.list.len() != before {
        repaint(guest);
    }
}
