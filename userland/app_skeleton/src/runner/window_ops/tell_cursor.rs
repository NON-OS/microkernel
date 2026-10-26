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
//! The pointer's shape over this window, told to the compositor when the
//! frame under the pointer wants another: a resize arrow over an edge or a
//! corner, the arrow elsewhere. Once a change, not once a move.

use crate::app::App;
use crate::clients::compositor;
use crate::discover::Peers;

use super::super::boot::BootedApp;
use super::super::request_id::next;

pub(in crate::runner) fn tell_cursor<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) {
    let (want, told) = booted.drag.cursor;
    if want == told {
        return;
    }
    let handle = booted.binding.surface_handle;
    if compositor::cursor_shape(peers.compositor, next(request_id), handle, want).is_ok() {
        booted.drag.cursor.1 = want;
    }
}
