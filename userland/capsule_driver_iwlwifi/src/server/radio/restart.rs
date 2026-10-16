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

//! The firmware restarted after it raised its error cause: the device
//! stopped, Bus Master off and the link noted lost, then after a rest Bus
//! Master on and the firmware booted again in the same regions, with the
//! same station address. net.core rejoins a lost link; the tries are bounded
//! (`Restarts`), and a radio whose tries are spent stays down.

use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::start::stop_device;
use crate::firmware::gen3::sweep::Sweep;

use super::bring::Rig;
use super::clock::Uptime;
use super::restarts::Restarts;
use super::state::{Radio, State};

/// A restart waiting for its rest to end.
#[derive(Clone, Copy)]
pub(super) struct Restart {
    pub(super) rig: &'static Rig,
    pub(super) addr: Option<[u8; 6]>,
    pub(super) at_ms: u64,
    pub(super) restarts: Restarts,
}

impl Radio {
    /// The firmware raised its error cause at `now`: stop it and note the
    /// link lost, then wait to restart it, or stay down once the tries are
    /// spent.
    pub(super) fn firmware_failed(&mut self, now: u64) {
        let State::Up(up) = &self.state else { return };
        stop_device(up.dev.m, &mut Uptime);
        let (rig, addr, restarts) = (up.rig, up.addr, up.restarts);
        self.master_off();
        self.after_firmware_stopped();
        self.wait_or_stay_down(Restart { rig, addr, at_ms: now, restarts }, Failure::Lost, now);
    }

    /// The joined link ends, its queues and inbox start afresh, and the
    /// sweep in flight with the old firmware is dropped (its counts kept).
    fn after_firmware_stopped(&mut self) {
        self.join.restart();
        let (frames, completed, stalled) =
            (self.sweep.frames, self.sweep.completed, self.sweep.stalled);
        self.sweep = Sweep::new();
        (self.sweep.frames, self.sweep.completed, self.sweep.stalled) =
            (frames, completed, stalled);
    }
}
