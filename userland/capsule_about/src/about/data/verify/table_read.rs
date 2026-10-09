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

//! Filling the tally from the kernel.

use alloc::vec::Vec;

use super::procs::{each, name_of};
use super::table::{Tally, INIT_NAME};

pub fn read(me: u32) -> Option<Tally> {
    let mut t =
        Tally { masks: Vec::new(), is_init: Vec::new(), total: 0, unmasked: 0, own_mask: 0 };
    let answered = each(|e| {
        t.masks.push(e.caps);
        t.is_init.push(name_of(e) == INIT_NAME);
        t.total += 1;
        if e.caps == 0 {
            t.unmasked += 1;
        }
        // 0 is the kernel's "unknown" answer for a pid, and no process has it.
        if me != 0 && e.pid == me {
            t.own_mask = e.caps;
        }
    });
    answered.then_some(t)
}
