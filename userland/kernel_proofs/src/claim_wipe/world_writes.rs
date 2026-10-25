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

//! Config space as the model's function takes its writes: Command keeps only
//! the bits the function has; Initiate FLR resets it if it truly offers an
//! FLR; leaving D3hot resets it unless it keeps its state there (recalled,
//! unverified); a wipe's own D3hot moves open a window (I3) and its D3hot
//! write counts as a reset write (I4).

use super::bits_rule::{BME, D3HOT, INITIATE_FLR, INTX_DISABLE, MEM, PM_STATE};
use super::world::World;
use super::world_cases::QUIET_PM;

impl World {
    /// The function keeps only the Command bits it has.
    pub(super) fn update_command(&mut self, wipe: bool, f: impl FnOnce(u16) -> u16) {
        self.touch(wipe);
        let d = &mut self.dev;
        let stuck = if d.stuck_bm { d.cmd & BME } else { 0 };
        let gone =
            if d.intx_sticks { 0 } else { INTX_DISABLE } | if d.caps.mem_bar { 0 } else { MEM };
        d.cmd = (f(d.cmd) | stuck) & !gone;
    }

    pub(super) fn set_devctl(&mut self, wipe: bool, v: u16) {
        self.touch(wipe);
        self.dev.devctl = v & !INITIATE_FLR;
        if v & INITIATE_FLR != 0 {
            self.flr(self.dev.flr);
        }
    }

    pub(super) fn af_flr(&mut self) {
        self.touch(true);
        self.flr(self.dev.af_flr);
    }

    /// D3hot entered or left: a wipe's write opens its window; leaving it
    /// resets the function unless it keeps its state through D3hot.
    pub(super) fn set_pmcsr(&mut self, wipe: bool, v: u16) {
        self.touch(wipe);
        let (from, to) = (self.dev.pm_state, v & PM_STATE);
        self.dev.pm_state = to;
        if wipe && from != to {
            self.dev.quiet_until = self.now + QUIET_PM;
        }
        if wipe && to == D3HOT && from != D3HOT {
            self.reset_write();
        }
        let soft = self.dev.soft_d3 && !self.dev.ignores_d3;
        if from == D3HOT && to == 0 && soft {
            self.reset_now(wipe);
        }
    }
}
