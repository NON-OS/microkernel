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

//! Build an association request: capability information, a listen interval,
//! then `elements`, each already a whole element (id, length, content), in
//! the order given (IEEE Std 802.11-2020, Table 9-34: SSID, Supported
//! Rates, Extended Supported Rates, RSN, then the RSN Extension element).

use super::*;

/// Build an association request: capability information, a listen interval,
/// then `elements`, each already a whole element (id, length, content), in
/// the order given (IEEE Std 802.11-2020, Table 9-34: SSID, Supported Rates,
/// Extended Supported Rates, RSN, then the RSN Extension element).
pub fn assoc_request_elements(
    out: &mut [u8],
    src: MacAddr,
    bssid: MacAddr,
    cap: u16,
    seq: u16,
    elements: &[&[u8]],
) -> Option<usize> {
    let fc = frame_control(TYPE_MGMT, SUBTYPE_ASSOC_REQ);
    let off = write_header(out, fc, bssid, src, bssid, seq)?;
    let mut cur = off.checked_add(4)?;
    if cur > out.len() {
        return None;
    }
    out[off..off + 2].copy_from_slice(&cap.to_le_bytes());
    out[off + 2..off + 4].copy_from_slice(&10u16.to_le_bytes());
    for e in elements {
        let end = cur.checked_add(e.len())?;
        out.get_mut(cur..end)?.copy_from_slice(e);
        cur = end;
    }
    Some(cur)
}
