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

//! The networks a scan finds, shared by both drivers (it began in the
//! RTL8821CE driver). The channel hop or the firmware scan feeds it beacons;
//! this is the pure collection: deduplicate by access point, drop hidden
//! networks, cap the list, and encode in the format the settings panel parses:
//! a count, then per network a signal byte (0 to 100), a flags byte, the SSID
//! length and the SSID. The flags say secured (bit 0), and which personal
//! security the RSN element offers: WPA2 (bit 1: PSK or PSK-SHA256) and WPA3
//! (bit 2: SAE), so the panel can show a WPA3 network as such and the client
//! remember it as WPA3.
//!
//! The picture ages: each full channel sweep is counted, a network not heard
//! for `MAX_AGE` sweeps is dropped, and when the list is full a network heard
//! now replaces the one heard longest ago. Without that the first sixteen
//! networks ever heard stayed listed for the whole boot, gone or not, and any
//! seventeenth was never shown. Kept free of hardware so it is checked on the
//! host.

/// The most networks a scan reports.
pub const MAX_RESULTS: usize = 16;

/// Sweeps a network may go unheard before it is dropped.
pub const MAX_AGE: u32 = 3;

pub(super) const SSID_MAX: usize = 32;

pub(super) const FLAG_SECURED: u8 = 0x01;

/// The network offers WPA2-Personal.
pub const FLAG_WPA2: u8 = 0x02;

/// The network offers WPA3-Personal (SAE).
pub const FLAG_WPA3: u8 = 0x04;

#[derive(Clone, Copy)]
pub(super) struct Entry {
    pub(super) ssid: [u8; SSID_MAX],
    pub(super) ssid_len: u8,
    pub(super) bssid: [u8; 6],
    pub(super) flags: u8,
    pub(super) signal: u8,
    pub(super) seen: u32,
}

/// The networks a scan found, deduplicated by access point.
pub struct ScanResults {
    pub(super) entries: [Entry; MAX_RESULTS],
    pub(super) count: usize,
    pub(super) sweep: u32,
}

impl Default for ScanResults {
    fn default() -> Self {
        Self::new()
    }
}
