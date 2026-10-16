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

//! Build a reassociation request (IEEE Std 802.11-2020, 9.3.3.7): the
//! association request's capability and listen interval, then the address
//! of the AP the station is moving from, then the same elements in the same
//! order. The current AP's address lets the new AP and the distribution
//! system move the station's traffic without waiting for the old AP to time
//! it out.

use super::*;
use crate::dot11::header::SUBTYPE_REASSOC_REQ;

pub fn reassoc_request_elements(
    out: &mut [u8],
    src: MacAddr,
    bssid: MacAddr,
    current_ap: MacAddr,
    cap: u16,
    seq: u16,
    elements: &[&[u8]],
) -> Option<usize> {
    let fc = frame_control(TYPE_MGMT, SUBTYPE_REASSOC_REQ);
    let off = write_header(out, fc, bssid, src, bssid, seq)?;
    let fixed = out.get_mut(off..off.checked_add(10)?)?;
    fixed[..2].copy_from_slice(&cap.to_le_bytes());
    fixed[2..4].copy_from_slice(&10u16.to_le_bytes());
    fixed[4..10].copy_from_slice(&current_ap);
    let mut cur = off + 10;
    for e in elements {
        let end = cur.checked_add(e.len())?;
        out.get_mut(cur..end)?.copy_from_slice(e);
        cur = end;
    }
    Some(cur)
}
