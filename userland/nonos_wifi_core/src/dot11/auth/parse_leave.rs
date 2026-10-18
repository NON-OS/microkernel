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

//! The reason code of a Deauthentication or Disassociation frame the access
//! point `bssid` sent to `us` (or to everyone), or `None` for any other
//! frame.

use super::*;

/// The reason code of a Deauthentication or Disassociation frame the access
/// point `bssid` sent to `us` (or to everyone), or `None` for any other frame.
pub fn parse_leave(frame: &[u8], us: &MacAddr, bssid: &MacAddr) -> Option<u16> {
    if frame.len() < MAC_HEADER_LEN + 2 {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    let sub = fc_subtype(fc);
    if fc_type(fc) != TYPE_MGMT || (sub != SUBTYPE_DEAUTH && sub != SUBTYPE_DISASSOC) {
        return None;
    }
    let to = &frame[4..10];
    if (to != &us[..] && to != &BROADCAST[..]) || frame[10..16] != bssid[..] {
        return None;
    }
    Some(u16::from_le_bytes([frame[MAC_HEADER_LEN], frame[MAC_HEADER_LEN + 1]]))
}

/// Build an Authentication frame to `bssid` with the given algorithm,
/// transaction sequence, status code and body.
pub fn auth_frame(
    src: MacAddr,
    bssid: MacAddr,
    seq_ctrl: u16,
    algorithm: u16,
    seq: u16,
    status: u16,
    body: &[u8],
) -> Vec<u8> {
    let mut out = alloc::vec![0u8; MAC_HEADER_LEN + 6 + body.len()];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_AUTH);
    // The buffer is sized for the header, so this cannot refuse.
    let _ = write_header(&mut out, fc, bssid, src, bssid, seq_ctrl);
    let f = &mut out[MAC_HEADER_LEN..];
    f[0..2].copy_from_slice(&algorithm.to_le_bytes());
    f[2..4].copy_from_slice(&seq.to_le_bytes());
    f[4..6].copy_from_slice(&status.to_le_bytes());
    f[6..].copy_from_slice(body);
    out
}

/// Build a Deauthentication frame to `bssid` with `reason`, unprotected.
pub fn deauth_frame(src: MacAddr, bssid: MacAddr, seq_ctrl: u16, reason: u16) -> Vec<u8> {
    let mut out = alloc::vec![0u8; MAC_HEADER_LEN + 2];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_DEAUTH);
    let _ = write_header(&mut out, fc, bssid, src, bssid, seq_ctrl);
    out[MAC_HEADER_LEN..].copy_from_slice(&reason.to_le_bytes());
    out
}
