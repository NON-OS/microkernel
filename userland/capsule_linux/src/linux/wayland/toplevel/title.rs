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
//! xdg_toplevel.set_title: kept with its toplevel's surface, which its
//! window's frame takes whenever it is shown (window_front/), and shown in
//! the title bar at once when that window is the one shown.

use crate::linux::guest::Guest;

use super::super::present::frame::redraw::{title as retitle, trimmed};

pub fn title(guest: &mut Guest, toplevel: u32, title: &[u8]) {
    let title = trimmed(title);
    let scene = &mut guest.scene;
    let Some(s) = scene.surfaces.iter_mut().find(|s| s.toplevel == Some(toplevel)) else { return };
    s.title.clear();
    s.title.extend_from_slice(title);
    let (surface, shown) = (s.id, scene.win.shown.map(|w| w.surface));
    // The first window takes it as it opens (window_front/front.rs).
    if shown == Some(surface) {
        retitle(scene, title);
    }
}
