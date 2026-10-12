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

//! The header layout of a received MPDU, or `None` if it is shorter than
//! the header its frame control describes.

use super::*;

/// The header layout of a received MPDU, or `None` if it is shorter than the
/// header its frame control describes.
pub fn view(frame: &[u8]) -> Option<FrameView> {
    if frame.len() < MAC_HEADER_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    let mut hdr_len = MAC_HEADER_LEN;
    if fc & FC_TO_DS != 0 && fc & FC_FROM_DS != 0 {
        hdr_len += 6;
    }
    let mut qos_at = None;
    let mut tid = 0;
    if fc_type(fc) == TYPE_DATA && fc_subtype(fc) & SUBTYPE_QOS != 0 {
        qos_at = Some(hdr_len);
        tid = *frame.get(hdr_len)? & 0x0F;
        hdr_len += 2;
        if fc & FC_ORDER != 0 {
            hdr_len += HT_CONTROL_LEN;
        }
    }
    if frame.len() < hdr_len {
        return None;
    }
    Some(FrameView { fc, hdr_len, qos_at, tid })
}

/// The packet number and key index from the CCMP header after `hdr_len`, or
/// `None` if the frame cannot hold the header and a MIC or ExtIV is clear.
pub fn pn_and_key(frame: &[u8], hdr_len: usize) -> Option<(u64, u8)> {
    let end = hdr_len.checked_add(CCMP_HDR_LEN)?;
    if frame.len() < end.checked_add(MIC_LEN)? {
        return None;
    }
    let h = &frame[hdr_len..end];
    if h[3] & EXT_IV == 0 {
        return None;
    }
    let pn = (h[0] as u64)
        | ((h[1] as u64) << 8)
        | ((h[4] as u64) << 16)
        | ((h[5] as u64) << 24)
        | ((h[6] as u64) << 32)
        | ((h[7] as u64) << 40);
    Some((pn, h[3] >> 6))
}
