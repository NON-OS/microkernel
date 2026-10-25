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

//! The firmware restart once its rest ends.

use crate::firmware::gen3::outcome::master_on;
use crate::setup::command::command;

use super::bring::{boot_rig, finish};
use super::clock::Uptime;
use super::say::Line;
use super::state::{Radio, State};

impl Radio {
    /// Once the rest ends: the device stopped and seen stopped, then Bus
    /// Master on, then the firmware booted and set up in the same regions.
    /// A step that fails stopped the device (the boot and set-up stop it),
    /// so Bus Master goes off before the next rest.
    pub(super) fn restart_due(&mut self, now: u64) {
        let State::Restarting(r) = &self.state else { return };
        if now < r.at_ms {
            return;
        }
        let r = *r;
        let (id, epoch) = self.pci;
        let up = master_on(r.rig.regs, &mut Uptime, || command(id, epoch, true))
            .and_then(|()| boot_rig(r.rig))
            .and_then(|dev| finish(dev, r.rig, r.addr, r.restarts));
        match up {
            Ok(up) => {
                Line::new().text(b"radio up again; passive scanning").send();
                self.state = State::Up(up);
                self.next_ms = now;
            }
            Err(f) => {
                self.master_off();
                self.wait_or_stay_down(r, f, now);
            }
        }
    }
}
