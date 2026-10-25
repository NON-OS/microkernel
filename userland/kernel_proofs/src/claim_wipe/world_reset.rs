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

//! What a reset does to the function, and what a reset write must respect:
//! not over transactions still pending inside their bound, and one write a
//! pass however many steppers contend (I4).

use super::bits_rule::BME;
use super::phase_rule::MS;
use super::world::World;
use super::world_cases::{QUIET_FLR, TP_HOLD};

impl World {
    /// An FLR asked; `has`: the function truly offers that one.
    pub(super) fn flr(&mut self, has: bool) {
        self.reset_write();
        self.dev.quiet_until = self.now + QUIET_FLR;
        if has && !self.dev.ignores_flr {
            self.reset_now(true);
        }
    }

    pub(super) fn reset_write(&mut self) {
        self.truth.writes = self.truth.writes.saturating_add(1);
        if self.truth.writes > 1 {
            self.bad(4, "two reset writes in one pass");
        }
        let early = self.now.saturating_sub(self.truth.owed_at) < TP_HOLD;
        if early && self.now < self.dev.tp_until {
            self.bad(4, "a reset written over Transactions Pending");
        }
    }

    /// The function reset: back at 0 to 1.3 s, or never, after a reset's own
    /// write (`slow`), at once after a claim's power-on. A display's memory
    /// outlives it (recalled, unverified); so does what a function past Bus
    /// Master reaches.
    pub(super) fn reset_now(&mut self, slow: bool) {
        let back = match (slow, self.draw(16)) {
            (false, _) => self.now,
            (true, 0) => u64::MAX,
            _ => self.now + MS * self.draw(1301),
        };
        let d = &mut self.dev;
        d.cmd &= if d.stuck_bm { BME } else { 0 };
        (d.devctl, d.pm_state, d.tp_until, d.ready_at) = (0, 0, 0, back);
        d.secret = d.secret.filter(|_| d.caps.display);
        let reach = d.past_bm;
        d.queue.retain(|_| reach);
    }

    /// Bus Master off (quiesce.rs, master_off.rs): the first time after a
    /// tenancy, what it has pending finishes within 300 ms, or never.
    pub(super) fn stop_bme(&mut self) {
        self.update_command(false, |v| v & !BME);
        if self.dev.serving && (self.dev.caps.pcie || self.dev.caps.af_tp) {
            let left = MS * self.draw(301);
            self.dev.tp_until = if self.dev.tp_never { u64::MAX } else { self.now + left };
        }
        self.dev.serving = false;
    }
}
