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

//! A window's size now, in the logical pixels its configure speaks: the
//! geometry it set, else its buffer's size at its scale, or its viewport's.
//! A configure that changes only its states says this size again; told 0 by
//! 0, GTK takes its least size (LibreOffice's main window went to 24 by 26).

use crate::linux::guest::Guest;

use super::super::args::Args;

/// xdg_surface.set_window_geometry; one with no area is the client's
/// invalid_size, and is not taken.
pub fn geometry(guest: &mut Guest, xdg: u32, args: &mut Args<'_>) {
    let mut w = [0i32; 4];
    for v in w.iter_mut() {
        *v = args.u32().unwrap_or(0) as i32;
    }
    if w[2] <= 0 || w[3] <= 0 {
        return;
    }
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.xdg == Some(xdg)) {
        s.geometry = Some((w[0], w[1], w[2], w[3]));
    }
}

/// The size `toplevel`'s window is drawn at, once it has drawn one.
pub fn current(guest: &Guest, toplevel: u32) -> Option<(u32, u32)> {
    let s = guest.scene.surfaces.iter().find(|s| s.toplevel == Some(toplevel))?;
    if let Some((_, _, w, h)) = s.geometry {
        return Some((w as u32, h as u32));
    }
    if let Some(dest) = s.scale.dest {
        return Some(dest);
    }
    let b = guest.scene.buffers.iter().find(|b| Some(b.id) == s.pending)?;
    let k = s.scale.buffer.max(1);
    Some((b.width / k, b.height / k))
}
