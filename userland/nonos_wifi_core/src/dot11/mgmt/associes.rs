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

//! The information elements a station advertises in an association request:
//! its SSID, its supported rates, and the complete RSN element that tells a
//! WPA2 AP it will do CCMP.

use super::*;

/// The information elements a station advertises in an association request: its
/// SSID, its supported rates, and the complete RSN element that tells a WPA2 AP
/// it will do CCMP. Grouped so the frame builder takes the payload as one value
/// rather than a long argument list.
pub struct AssocIes<'a> {
    pub ssid: &'a [u8],
    pub rates: &'a [u8],
    pub rsn: &'a [u8],
}

/// Build an association request: capability info, a listen interval, then the
/// SSID, supported-rates, and RSN elements from `ies`. `bssid` is the AP being
/// joined.
pub fn assoc_request(
    out: &mut [u8],
    src: MacAddr,
    bssid: MacAddr,
    ies: &AssocIes,
    cap: u16,
    seq: u16,
) -> Option<usize> {
    let fc = frame_control(TYPE_MGMT, SUBTYPE_ASSOC_REQ);
    let off = write_header(out, fc, bssid, src, bssid, seq)?;
    let body = off + 4;
    if body > out.len() {
        return None;
    }
    out[off..off + 2].copy_from_slice(&cap.to_le_bytes());
    out[off + 2..off + 4].copy_from_slice(&10u16.to_le_bytes());
    let mut cur = body;
    cur = put_ie(out, cur, IE_SSID, ies.ssid)?;
    cur = put_ie(out, cur, IE_SUPPORTED_RATES, ies.rates)?;
    // The RSN element is already a complete element (id, length, then the cipher
    // and AKM suites), so it is written verbatim rather than through put_ie. A
    // WPA2 access point will not complete association for a station that omits it.
    let end = cur + ies.rsn.len();
    if end > out.len() {
        return None;
    }
    out[cur..end].copy_from_slice(ies.rsn);
    Some(end)
}

/// Build an element `[id][len][data]` into `out` at `off`, returning the new
/// offset, or `None` on overflow. The association builder below takes whole
/// elements; this is how the MLME makes the SSID and rates ones.
pub fn element(out: &mut [u8], off: usize, id: u8, data: &[u8]) -> Option<usize> {
    put_ie(out, off, id, data)
}
