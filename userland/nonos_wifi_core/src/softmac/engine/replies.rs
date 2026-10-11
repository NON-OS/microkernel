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

//! The answers that need no air time. Status: the bring-up stage, then the
//! data-path counters, the chip's diagnostic words, the refusals and rekeys.
//! Link: `[associated u8][bssid 6][ssid_len u8][ssid][akm u8]`, the AKM
//! telling the client to remember a WPA3 network as WPA3. Scan: the
//! scanner's three counters, then the networks it has heard.

use super::station::SoftStation;
use crate::softmac::radio::{Env, Radio};

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub(super) fn status_reply(&self, out: &mut [u8]) -> usize {
        let s = self.stats;
        let diag = self.radio.as_ref().map_or([0; 5], Radio::diag);
        out[0] = self.stage;
        let words = [s.tx_ok, s.tx_drop, s.rx_ring, s.rx_eth, s.netif_reqs, s.rx_err];
        let tail = [s.rx_refused, s.rekeys];
        for (i, w) in words.iter().chain(diag.iter()).chain(tail.iter()).enumerate() {
            out[1 + 4 * i..5 + 4 * i].copy_from_slice(&w.to_le_bytes());
        }
        1 + 4 * (words.len() + diag.len() + tail.len())
    }

    pub(super) fn link_reply(&self, out: &mut [u8]) -> usize {
        out[..8].fill(0);
        let Some(s) = self.session.as_ref().filter(|s| s.is_up()) else {
            return 8;
        };
        let name = s.ssid();
        out[0] = 1;
        out[1..7].copy_from_slice(&s.bssid);
        out[7] = name.len() as u8;
        out[8..8 + name.len()].copy_from_slice(name);
        out[8 + name.len()] = s.akm;
        9 + name.len()
    }

    pub(super) fn scan_reply(&self, out: &mut [u8]) -> usize {
        if self.radio.is_none() {
            out[..13].fill(0);
            return 13;
        }
        let sc = &self.scanner;
        for (i, w) in [sc.steps, sc.raw, sc.beacons].iter().enumerate() {
            out[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
        }
        12 + sc.results().encode(&mut out[12..])
    }
}
