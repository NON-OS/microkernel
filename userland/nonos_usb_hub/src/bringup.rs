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

//! Bringing up the device on a hub port without holding the walk (USB 2.0
//! sections 7.1.7.3, 7.1.7.5 and 11.5.1.5, as recalled, unverified). Each
//! look at the port takes one step (`Stage`); the debounce, the reset and
//! the recovery after it are deadlines, not sleeps, so the other hubs' ports
//! are looked at between steps, and a port is reset only as often as its
//! `Budget` allows. A flapping or hostile hub had the driver sleep through a
//! debounce and a reset on every port at every pass.

use crate::budget::Budget;
use crate::port_status::HubPort;
use crate::stage::Stage;
use crate::step::Step;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bringup {
    stage: Stage,
    /// The device here was addressed: not tried again until it leaves or
    /// another takes its place.
    decided: bool,
    budget: Budget,
}

impl Bringup {
    pub const fn new() -> Self {
        Self { stage: Stage::Idle, decided: false, budget: Budget::new() }
    }

    /// Mid way through: the port wants looking at again soon.
    pub fn busy(&self) -> bool {
        self.stage != Stage::Idle
    }

    /// The device was addressed and classified, whatever bound.
    pub fn decide(&mut self) {
        self.decided = true;
    }

    /// The hub refused the step asked of it, or its port could not be
    /// read: this try ends, and counts.
    pub fn end_try(&mut self) {
        self.stage = Stage::Idle;
    }

    /// Reset, or recovering from it (`Stage::resetting`).
    pub fn resetting(&self) -> bool {
        self.stage.resetting()
    }

    /// One look at the port; a debounced device is reset only when
    /// `may_reset`, so one device at a time answers at address 0.
    pub fn step(&mut self, st: HubPort, now_ms: u64, may_reset: bool) -> Step {
        if !st.connected {
            self.decided = false;
            return self.stage.gone();
        }
        // A device swapped in between two looks shows only as a change of
        // connection: it is a new device, not the one decided on.
        if st.connect_changed {
            self.decided = false;
        }
        // A try is a reset: one that waits its turn spends none.
        if self.stage == Stage::Idle && (self.decided || !self.budget.left(now_ms)) {
            return Step::Wait;
        }
        let step = self.stage.step(st, now_ms, may_reset);
        if step == Step::Reset {
            self.budget.spend(now_ms);
        }
        step
    }
}
