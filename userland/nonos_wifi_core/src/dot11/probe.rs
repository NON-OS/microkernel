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

//! A probe request addressed to the access point a station is joined to, to
//! learn whether it is still there: it answers with a probe response to this
//! station. The SSID is the network's own, since an access point with a
//! hidden SSID does not answer a wildcard one.

use alloc::vec::Vec;

use super::header::{
    frame_control, write_header, MacAddr, MAC_HEADER_LEN, SUBTYPE_PROBE_REQ, TYPE_MGMT,
};
use super::mgmt::{IE_SSID, IE_SUPPORTED_RATES};

/// 1, 2, 5.5 and 11 Mb/s (basic), then 6, 9, 12 and 18: rates every access
/// point on either band takes in a probe.
const RATES: [u8; 8] = [0x82, 0x84, 0x8B, 0x96, 0x0C, 0x12, 0x18, 0x24];

/// The probe request from `src` to `bssid` for the network `ssid`, or `None`
/// for an SSID longer than an element holds.
pub fn probe_to(src: MacAddr, bssid: MacAddr, ssid: &[u8], seq: u16) -> Option<Vec<u8>> {
    if ssid.len() > 32 {
        return None;
    }
    let mut out = alloc::vec![0u8; MAC_HEADER_LEN + 2 + ssid.len() + 2 + RATES.len()];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_PROBE_REQ);
    let mut at = write_header(&mut out, fc, bssid, src, bssid, seq)?;
    for (id, data) in [(IE_SSID, ssid), (IE_SUPPORTED_RATES, &RATES[..])] {
        out[at] = id;
        out[at + 1] = data.len() as u8;
        out[at + 2..at + 2 + data.len()].copy_from_slice(data);
        at += 2 + data.len();
    }
    Some(out)
}
