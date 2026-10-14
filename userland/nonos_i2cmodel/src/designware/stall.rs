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

//! A bus that stops moving. The core takes commands and runs none of them,
//! and its status reads busy while any are queued: the master's own state
//! machine wedged by a glitch, or a device holding SDA low. Holding the LPSS
//! function in reset returns the core to its power-on state; whether the bus
//! moves again afterwards depends on which of the two stopped it.

use super::regs::LPSS_RESETS_RELEASED;
use super::state::Designware;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stall {
    None,
    /// The master wedged: a function reset clears it.
    UntilReset,
    /// A device holds the bus: no reset of the master frees it.
    ForGood,
}

impl Designware {
    /// Stop the bus from now on: until the function is next held in reset
    /// when `a_reset_frees_it`, for good otherwise.
    pub fn stall_bus(&mut self, a_reset_frees_it: bool) {
        self.stall = if a_reset_frees_it { Stall::UntilReset } else { Stall::ForGood };
    }

    pub(super) fn stalled(&self) -> bool {
        self.stall != Stall::None
    }

    /// LPSS_PRIV_RESETS written. While the function is held, every core
    /// register is back at its power-on value and the queues are empty.
    pub(super) fn lpss_resets(&mut self, value: u32) {
        self.resets = value;
        if value & LPSS_RESETS_RELEASED == LPSS_RESETS_RELEASED {
            return;
        }
        self.enable = 0;
        self.tx.clear();
        self.rx.clear();
        self.active = false;
        self.dir = None;
        self.regs.clear();
        self.abort_source = 0;
        self.raw_intr = 0;
        if self.stall == Stall::UntilReset {
            self.stall = Stall::None;
        }
    }
}
