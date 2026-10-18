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
use super::mgmt_rx::SaTick;
use super::{LinkStation, Loss, Tick};
use crate::dot11::probe::probe_to;
use crate::pmf::SaQuery;

impl LinkStation {
    /// Advance the clock to `now_ms` and do what is due.
    pub fn tick(&mut self, now_ms: u64) -> Tick {
        if !self.associated {
            return Tick::Quiet;
        }
        let now = self.watch.clock.map_or(now_ms, |c| c.max(now_ms));
        self.watch.clock = Some(now);
        if let Some(t) = self.query_due(now) {
            return t;
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

    // The station's SA Query at `now`: a retry with a new identifier, or
    // the link ended unanswered. One begun before the clock ran starts now.
    fn query_due(&mut self, now: u64) -> Option<Tick> {
        if let Some(first) = self.watch.unstamped.take() {
            let running = self.pmf.query.is_some();
            self.pmf.query = running.then(|| SaQuery::start(now, first));
            return None;
        }
        match self.sa_query_poll(now, self.watch.txid()) {
            SaTick::Idle => None,
            SaTick::Send(frame) => {
                self.watch.took_txid();
                Some(Tick::Send(frame))
            }
            SaTick::Ended => Some(Tick::Lost(Loss::Unanswered(self.watch.asked.unwrap_or(0)))),
        }
    }
}
