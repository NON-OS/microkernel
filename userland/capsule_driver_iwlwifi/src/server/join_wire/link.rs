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

//! The link reply and a status-only reply.

use super::codes::{SSID_MAX, WIFI_HDR};

/// The association a link reply describes.
pub struct LinkInfo<'a> {
    pub bssid: [u8; 6],
    pub ssid: &'a [u8],
    /// The AKM suite type that ran (2 WPA2-PSK, 6 PSK-SHA256, 8 SAE).
    pub akm: u8,
}

/// Write the link reply after the header: not associated, or `link`.
pub fn encode_link(out: &mut [u8], link: Option<&LinkInfo<'_>>) -> Option<usize> {
    let fixed = WIFI_HDR + 8;
    out.get_mut(WIFI_HDR..fixed)?.fill(0);
    let Some(l) = link else { return Some(fixed) };
    let len = l.ssid.len().min(SSID_MAX);
    out[WIFI_HDR] = 1;
    out[WIFI_HDR + 1..WIFI_HDR + 7].copy_from_slice(&l.bssid);
    out[fixed - 1] = len as u8;
    out.get_mut(fixed..fixed + len)?.copy_from_slice(&l.ssid[..len]);
    *out.get_mut(fixed + len)? = l.akm;
    Some(fixed + len + 1)
}

/// Write a status-only reply after the header.
pub fn encode_code(out: &mut [u8], code: i32) -> Option<usize> {
    out.get_mut(WIFI_HDR..WIFI_HDR + 4)?.copy_from_slice(&code.to_le_bytes());
    Some(WIFI_HDR + 4)
}
