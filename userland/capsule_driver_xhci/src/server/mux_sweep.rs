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

//! Giving back what clients that exited held, on every controller
//! (`sweep_orphans`), and the multiplexer's slot ids that named it or that
//! such a client had only reserved.

use nonos_libc::mk_pid_alive;

use super::mux::Mux;
use super::mux_forward::VSlot;
use super::sweep::sweep_orphans;

impl Mux {
    pub fn sweep(&mut self) {
        for ctx in self.ctxs.iter_mut() {
            sweep_orphans(ctx);
        }
        for v in self.vslots.iter_mut() {
            let gone = match *v {
                VSlot::Pending { owner } => owner != 0 && !mk_pid_alive(owner),
                VSlot::Bound { ctrl, slot } => self
                    .ctxs
                    .get(ctrl as usize)
                    .is_some_and(|c| !c.driver.slots.is_allocated(slot, c.driver.layout.max_slots)),
                VSlot::Free => false,
            };
            if gone {
                *v = VSlot::Free;
            }
        }
    }
}
