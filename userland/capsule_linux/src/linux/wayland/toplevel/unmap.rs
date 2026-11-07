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

//! A toplevel unmapped by its client: a null buffer attached and committed,
//! as Qt hides a window it will show again. Its window is taken down, as
//! when its toplevel goes, its surface leaves the output, and its next
//! commit is an initial one again, answered with a first configure
//! (first.rs), as xdg-shell has it.

use crate::linux::guest::Guest;

use super::super::close::close_window::close_for;
use super::super::window_life::End;

pub fn unmap(guest: &mut Guest, surface: u32) {
    let s = guest.scene.surfaces.iter_mut().find(|s| s.id == surface && s.toplevel.is_some());
    let Some(s) = s.filter(|s| !s.initial) else { return };
    (s.initial, s.wants, s.configured) = (true, None, false);
    super::super::popup::ends::end_all(guest);
    close_for(guest, End::Surface(surface));
    super::super::output_enter::left(guest, surface);
}
