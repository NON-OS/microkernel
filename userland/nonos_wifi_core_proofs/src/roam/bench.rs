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

//! A radio for the look proofs: beacons waiting on each channel, the
//! channel tuned, the filter and every frame sent recorded.

use std::collections::HashMap;

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{Radio, RxInfo};

#[derive(Default)]
pub struct Bench {
    pub waiting: HashMap<u8, Vec<(Vec<u8>, i8)>>,
    pub tuned: Vec<u8>,
    pub bss: Option<[u8; 6]>,
    pub sent: Vec<Vec<u8>>,
}

impl Radio for Bench {
    fn mac(&self) -> [u8; 6] {
        [2, 0, 0, 0, 0, 2]
    }
    fn set_mac(&mut self, _: [u8; 6]) -> bool {
        false
    }
    fn bands(&self) -> (bool, bool) {
        (true, false)
    }
    fn tune(&mut self, _band: Band, channel: u8) -> bool {
        self.tuned.push(channel);
        true
    }
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.sent.push(mpdu.to_vec());
        true
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        let at = *self.tuned.last()?;
        let (f, rssi_dbm) = self.waiting.get_mut(&at)?.pop()?;
        out[..f.len()].copy_from_slice(&f);
        Some(RxInfo { len: f.len(), rssi_dbm, decrypted: false })
    }
    fn set_bss(&mut self, bssid: Option<[u8; 6]>) {
        self.bss = bssid;
    }
}
