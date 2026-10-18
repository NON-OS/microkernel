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

//! Record one beacon with only the secured bit (the form the earliest
//! proofs drive).

use super::*;

impl ScanResults {
    pub fn new() -> Self {
        let empty =
            Entry { ssid: [0; SSID_MAX], ssid_len: 0, bssid: [0; 6], flags: 0, signal: 0, seen: 0 };
        Self { entries: [empty; MAX_RESULTS], count: 0, sweep: 0 }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Record one beacon with only the secured bit (the form the earliest
    /// proofs drive).
    pub fn add(&mut self, bssid: [u8; 6], ssid: &[u8], secured: bool) {
        self.add_bss(bssid, ssid, if secured { FLAG_SECURED } else { 0 });
    }

    /// Record one beacon with its security flags and no signal reading.
    pub fn add_bss(&mut self, bssid: [u8; 6], ssid: &[u8], flags: u8) {
        self.heard(bssid, ssid, flags, 0);
    }
}
