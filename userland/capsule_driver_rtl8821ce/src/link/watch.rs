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

//! The link's clock, ticked from the serving loop: the station asks after a
//! quiet access point (its probes name the session's network) or answers an
//! SA Query, and a link whose access point is gone ends.

use nonos_wifi_core::station::Tick;

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Tick at `now_ms`: watch the transmit ring (`watch_tx`), then the
    /// station: send what it calls for, or note the end.
    pub fn tick(&mut self, now_ms: u64) {
        self.watch_tx(now_ms);
        match self.station.tick(now_ms) {
            Tick::Send(frame) => {
                self.send_raw(&frame);
            }
            Tick::Lost(why) => self.lost(why),
            Tick::Quiet => {}
        }
    }

    /// The network's SSID, for the station's probes.
    pub fn set_ssid(&mut self, ssid: &[u8]) {
        self.station.set_ssid(ssid);
    }
}
