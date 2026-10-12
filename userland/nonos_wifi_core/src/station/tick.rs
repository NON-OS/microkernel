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

//! The station's clock, ticked by the driver's serving loop: each tick sends
//! an SA Query request or ends one unanswered, then probes a quiet access
//! point or ends the link when none was answered. Time never runs backwards.

use super::alive::Due;
use super::query::Asked;
use super::watch::SSID_MAX;
use super::{LinkStation, Loss, Tick};
use crate::dot11::probe::probe_to;

impl LinkStation {
    /// Advance the clock to `now_ms` and do what is due.
    pub fn tick(&mut self, now_ms: u64) -> Tick {
        if !self.associated {
            return Tick::Quiet;
        }
        let now = self.watch.clock.map_or(now_ms, |c| c.max(now_ms));
        self.watch.clock = Some(now);
        match self.watch.query.due(now) {
            Asked::Unanswered(reason) => return Tick::Lost(self.lose(Loss::Unanswered(reason))),
            Asked::Ask(id) => {
                if let Some(frame) = self.sa_query(id, false) {
                    return Tick::Send(frame);
                }
            }
            Asked::Nothing => {}
        }
        match self.watch.alive.due(now) {
            Due::Nothing => Tick::Quiet,
            Due::Lost => Tick::Lost(self.lose(Loss::Silent)),
            Due::Probe => {
                let ssid = &self.watch.ssid[..self.watch.ssid_len];
                let probe = probe_to(self.our_mac, self.bssid, ssid, self.seq);
                self.seq = self.seq.wrapping_add(1) & 0x0FFF;
                probe.map_or(Tick::Quiet, Tick::Send)
            }
        }
    }

    /// The radio reports the access point's beacons missed: ask after it at
    /// the next tick rather than wait out the quiet.
    pub fn beacons_missed(&mut self) {
        if let Some(now) = self.watch.clock {
            self.watch.alive.missed(now);
        }
    }

    /// The network's SSID, for the probes; given after `associate`, which
    /// clears it. One longer than an SSID is ignored.
    pub fn set_ssid(&mut self, ssid: &[u8]) {
        if ssid.len() <= SSID_MAX {
            self.watch.ssid[..ssid.len()].copy_from_slice(ssid);
            self.watch.ssid_len = ssid.len();
        }
    }
}
