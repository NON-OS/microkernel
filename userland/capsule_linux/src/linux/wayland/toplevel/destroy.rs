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

//! `xdg_toplevel.destroy`.

use crate::linux::guest::Guest;

use super::super::close::close_window::close_for;
use super::super::object::Object;
use super::super::window_life::End;

/// xdg_toplevel.destroy: the window it made goes with it.
pub fn destroy(guest: &mut Guest, toplevel: u32) {
    super::super::popup::ends::end_all(guest);
    close_for(&mut guest.scene, End::Toplevel(toplevel));
    for s in guest.scene.surfaces.iter_mut().filter(|s| s.toplevel == Some(toplevel)) {
        s.toplevel = None;
    }
    super::super::forget::forget(guest, Some(Object::XdgToplevel), toplevel);
}
