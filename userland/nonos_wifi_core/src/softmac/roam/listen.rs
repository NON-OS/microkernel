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

//! What a look hears on one channel. A beacon of the session's network
//! from an AP not yet heard is kept, at most `HEARD_MAX`; one already heard
//! keeps its strongest signal. Every other frame is passed over.

use super::look::{Look, HEARD_MAX};
use super::pick::Heard;
use crate::dot11::parse::parse_beacon;
use crate::regdom::Band;
use crate::softmac::radio::Radio;

const DRAIN_PER_STEP: usize = 16;

impl Look {
    pub(super) fn listen<R: Radio>(&mut self, r: &mut R, ssid: &[u8], at: (Band, u8)) {
        let mut buf = [0u8; 2400];
        for _ in 0..DRAIN_PER_STEP {
            let Some(rx) = r.recv(&mut buf) else { break };
            let f = &buf[..rx.len.min(buf.len())];
            let Some(b) = parse_beacon(f).filter(|b| b.ssid == ssid) else { continue };
            let channel = if b.channel != 0 { b.channel } else { at.1 };
            match self.heard.iter().position(|h| h.bssid == b.bssid) {
                Some(i) => self.heard[i].rssi_dbm = self.heard[i].rssi_dbm.max(rx.rssi_dbm),
                None if self.heard.len() < HEARD_MAX => {
                    let (bssid, band, rssi_dbm, beacon) = (b.bssid, at.0, rx.rssi_dbm, f.to_vec());
                    self.heard.push(Heard { bssid, band, channel, rssi_dbm, beacon });
                }
                None => {}
            }
        }
    }
}
