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

//! The poll loop: the event page looked at every few milliseconds; when
//! the host flags events they are read and cleared, and a position change
//! is read and posted while the host's integration is on. With it off the
//! host moves the PS/2 pointer itself and nothing is posted here.

use nonos_libc::{input_abs_scale, mk_idle_ms};

use crate::constants::mouse::{EVENT_MOUSE_POSITION, HOST_WANTS_ABSOLUTE, POLL_MS, RANGE_MAX};
use crate::post::post_position;
use crate::req;
use crate::setup::Driver;

pub fn run(d: &mut Driver) -> ! {
    loop {
        if !step(d) {
            let _ = mk_idle_ms(POLL_MS);
        }
    }
}

/// One look. Whether the host had flagged anything.
pub fn step(d: &mut Driver) -> bool {
    if !d.memory.have_events() {
        return false;
    }
    let Ok(events) = req::acknowledge(&mut d.page) else { return true };
    if events & EVENT_MOUSE_POSITION == 0 {
        return true;
    }
    let Ok(m) = req::get_mouse(&mut d.page) else { return true };
    if m.features & HOST_WANTS_ABSOLUTE == 0 {
        return true;
    }
    let at = (input_abs_scale(m.x, 0, RANGE_MAX), input_abs_scale(m.y, 0, RANGE_MAX));
    if d.last != Some(at) && post_position(at) {
        d.last = Some(at);
    }
    true
}
