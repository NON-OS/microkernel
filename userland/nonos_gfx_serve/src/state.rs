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

//! STATE, and the fault path behind it (section 12; PLAN G 12): a lost
//! output (an underrun, a dropped link, a hung flip, a monitor replugged)
//! has its mode programmed again onto its current primary before the
//! answer, so the repaint the compositor does on LOST lands on a live
//! picture, with no restart of the driver. While that fails the output
//! answers LOST without SCANNING_OUT on every STATE and is tried again,
//! and the compositor may pick another mode from MODES meanwhile. An
//! output the device answers dark (neither flag: its monitor pulled) has
//! nothing to put the mode back on, so it rests until the device says LOST.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::{Errno, State};

use super::display::Display;
use super::platform::Platform;
use super::server::Server;

impl<D: Display, P: Platform> Server<D, P> {
    pub fn read_state(&mut self, output: u16) -> Result<State, Errno> {
        let mut st = self.display.state(output)?;
        let slot = self.slot(output)?;
        let cur = slot.current;
        st.serial = cur.map_or(0, |c| c.serial);
        if st.flags & (LOST | SCANNING_OUT) == 0 {
            slot.lost = false;
        }
        if st.flags & LOST == 0 && !slot.lost {
            return Ok(st);
        }
        let Some(c) = cur else {
            // Nothing was shown here yet; the next SURFACE starts afresh.
            slot.lost = false;
            return Ok(st);
        };
        let back = self.with_shadow(c, |d, s| d.set_mode(output, c.width, c.height, s));
        self.slot(output)?.lost = back.is_err();
        st.flags |= LOST;
        if back.is_err() {
            st.flags &= !SCANNING_OUT;
        }
        Ok(st)
    }
}
