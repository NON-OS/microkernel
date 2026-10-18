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

//! Encode for the settings panel: `[count]` then per network
//! `[signal][flags] [ssid_len][ssid]`.

use super::*;

impl ScanResults {
    /// Encode for the settings panel: `[count]` then per network `[signal][flags]
    /// [ssid_len][ssid]`. Returns the number of bytes written; a network that
    /// would overflow `out` is dropped rather than truncated.
    pub fn encode(&self, out: &mut [u8]) -> usize {
        if out.is_empty() {
            return 0;
        }
        let mut o = 1;
        let mut written = 0u8;
        for e in &self.entries[..self.count] {
            let n = e.ssid_len as usize;
            if o + 3 + n > out.len() {
                break;
            }
            out[o] = e.signal;
            out[o + 1] = e.flags;
            out[o + 2] = e.ssid_len;
            out[o + 3..o + 3 + n].copy_from_slice(&e.ssid[..n]);
            o += 3 + n;
            written += 1;
        }
        out[0] = written;
        o
    }
}

/// A received signal in dBm as the panel's 0 to 100 scale (-100 dBm or
/// weaker is 0, -50 dBm or stronger is 100); 0 dBm means not measured.
pub fn signal_percent(dbm: i8) -> u8 {
    if dbm >= 0 {
        return 0;
    }
    (2 * (dbm as i16 + 100)).clamp(0, 100) as u8
}
