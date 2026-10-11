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

//! The air between the station and the simulated access point, behind the
//! `Radio` trait a chip driver implements: the AP is heard only on its own
//! channel, beacons every 100 ms of the simulated clock, and answers what the
//! station sends while tuned there.

pub use crate::soft::rig::SimAir;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{Radio, RxInfo};

pub const STA: [u8; 6] = crate::ap_sim::SPA;

impl Radio for SimAir {
    fn mac(&self) -> [u8; 6] {
        STA
    }

    fn bands(&self) -> (bool, bool) {
        (true, true)
    }

    fn tune(&mut self, band: Band, channel: u8) -> bool {
        self.tuned = Some((band, channel));
        true
    }

    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.sent.push(mpdu.to_vec());
        self.sent_on.push(self.tuned);
        if self.on_channel() {
            let replies = self.ap.on_frame(mpdu);
            self.inbox.extend(replies);
        }
        true
    }

    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        let now = self.clock.get();
        let f = match self.inbox.pop_front() {
            Some(f) => f,
            None if self.on_channel() && now >= self.beacon_at => {
                self.beacon_at = now + 100;
                self.ap.beacon()
            }
            None => return None,
        };
        out[..f.len()].copy_from_slice(&f);
        Some(RxInfo { len: f.len(), rssi_dbm: -50, decrypted: false })
    }

    fn set_bss(&mut self, bssid: Option<[u8; 6]>) {
        self.bss = bssid;
    }
}
