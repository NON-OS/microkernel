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

//! What tg3_timer does once a second for a polled part: ask for a fresh
//! status block, send ASF firmware its heartbeat every two seconds, and
//! look at the link.
//!
//! The link is set up again when BMSR's link bit disagrees with the last
//! setup, or when the status block flags a change while the link is up (a
//! renegotiation can flap faster than a tick). A change flagged while it
//! stays down is only acknowledged: setup with the link down restarts
//! autonegotiation, and doing that each tick would never let it finish.

use nonos_libc::Deadline;

use crate::chip::fw;
use crate::constants::timeouts::{ASF_HEARTBEAT_MS, TICK_MS};
use crate::init::coalesce;
use crate::link::note;
use crate::phy::{copper, link_bit, Mdio};
use crate::setup::Driver;

pub struct Ticker {
    next: Deadline,
    heartbeat: Deadline,
}

impl Ticker {
    pub fn new() -> Self {
        Self { next: Deadline::after_ms(TICK_MS), heartbeat: Deadline::after_ms(ASF_HEARTBEAT_MS) }
    }

    pub fn run(&mut self, d: &mut Driver) {
        if !self.next.expired() {
            return;
        }
        self.next = Deadline::after_ms(TICK_MS);
        coalesce::now(&d.regs);
        if self.heartbeat.expired() {
            self.heartbeat = Deadline::after_ms(ASF_HEARTBEAT_MS);
            fw::heartbeat(&d.regs, d.modes.asf);
        }
        let flagged = d.status.take_link_change();
        let up = link_bit(&Mdio { regs: d.regs }, 0);
        if up != d.modes.link_up || (flagged && up) {
            let state = copper::setup(d);
            note(&mut d.link, state);
        }
    }
}
