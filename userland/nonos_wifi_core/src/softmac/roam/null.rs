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

//! The frame that tells the AP the station is away (IEEE Std 802.11-2020,
//! 11.2.3): a Null data frame to the AP with Power Management set, so the
//! AP buffers the station's frames while it listens on another channel, and
//! one with Power Management clear on its return, so the AP sends them.
//! A Null frame carries no data, so it is sent in the clear.

/// Type data, subtype Null; To DS; Power Management when `away`.
pub fn pm_null(sta: [u8; 6], bssid: [u8; 6], away: bool) -> [u8; 24] {
    let mut f = [0u8; 24];
    f[0] = 0x48;
    f[1] = 0x01 | if away { 0x10 } else { 0 };
    f[4..10].copy_from_slice(&bssid);
    f[10..16].copy_from_slice(&sta);
    f[16..22].copy_from_slice(&bssid);
    f
}
