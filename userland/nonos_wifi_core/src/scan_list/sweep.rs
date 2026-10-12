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

//! Record one beacon with its security flags and signal (0 to 100).

use super::*;

impl ScanResults {
    /// Record one beacon with its security flags and signal (0 to 100). An
    /// access point already listed is refreshed (flags and signal updated); a
    /// new one is added, or, with the list full, replaces the one heard
    /// longest ago if that was in an earlier sweep. Hidden networks (an empty
    /// SSID) and overlong SSIDs are not listed.
    pub fn heard(&mut self, bssid: [u8; 6], ssid: &[u8], flags: u8, signal: u8) {
        if ssid.is_empty() || ssid.len() > SSID_MAX {
            return;
        }
        let sweep = self.sweep;
        if let Some(e) = self.entries[..self.count].iter_mut().find(|e| e.bssid == bssid) {
            e.flags = flags;
            e.signal = signal;
            e.seen = sweep;
            return;
        }
        let slot = if self.count < MAX_RESULTS {
            self.count += 1;
            self.count - 1
        } else {
            let oldest = (0..self.count).min_by_key(|&i| self.entries[i].seen).unwrap_or(0);
            if self.entries[oldest].seen >= sweep {
                return;
            }
            oldest
        };
        let mut e = Entry {
            ssid: [0; SSID_MAX],
            ssid_len: ssid.len() as u8,
            bssid,
            flags,
            signal,
            seen: sweep,
        };
        e.ssid[..ssid.len()].copy_from_slice(ssid);
        self.entries[slot] = e;
    }

    /// A channel sweep finished: count it and drop networks not heard for
    /// `MAX_AGE` sweeps.
    pub fn end_sweep(&mut self) {
        self.sweep = self.sweep.wrapping_add(1);
        let sweep = self.sweep;
        let mut i = 0;
        while i < self.count {
            if sweep.wrapping_sub(self.entries[i].seen) > MAX_AGE {
                self.count -= 1;
                self.entries[i] = self.entries[self.count];
            } else {
                i += 1;
            }
        }
    }
}
