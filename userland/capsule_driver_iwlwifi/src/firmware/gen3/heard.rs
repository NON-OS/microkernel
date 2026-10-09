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

//! A management frame the firmware passed up during a scan, into the scan
//! list both drivers share: a beacon or probe response is parsed by the
//! shared core (every length checked there), its security flags read from
//! its RSN element, and its signal put on the panel's 0 to 100 scale. Any
//! other frame, or one too short to parse, is not listed.

use nonos_wifi_core::dot11::parse::parse_beacon;
use nonos_wifi_core::scan_list::{beacon_flags, signal_percent, ScanResults};

use super::rx_frame::RxFrame;

/// Record `f` if it is a beacon or probe response; `true` when it was.
pub fn hear(results: &mut ScanResults, f: &RxFrame) -> bool {
    let Some(b) = parse_beacon(&f.frame) else { return false };
    results.heard(b.bssid, b.ssid, beacon_flags(&f.frame, b.capability), signal_percent(f.signal));
    true
}
