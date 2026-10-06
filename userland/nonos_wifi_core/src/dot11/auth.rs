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

//! Authentication and deauthentication frames in full (IEEE Std 802.11-2020,
//! 9.3.3.12 and 9.3.3.13): the algorithm, transaction sequence and status
//! code of an Authentication frame and whatever body follows them (SAE's
//! commit and confirm), and the reason code of a Deauthentication or
//! Disassociation frame. Each parser also checks who the frame is from and
//! for: a join listens only to its own access point, so another station's
//! refusal on the same channel cannot end it.

use alloc::vec::Vec;

use super::header::{
    fc_subtype, fc_type, frame_control, write_header, MacAddr, MAC_HEADER_LEN, SUBTYPE_AUTH,
    TYPE_MGMT,
};

/// Management subtypes for leaving.
pub const SUBTYPE_DISASSOC: u8 = 10;
pub const SUBTYPE_DEAUTH: u8 = 12;
/// The broadcast address, which a deauthentication may be sent to.
const BROADCAST: MacAddr = [0xFF; 6];

/// A parsed Authentication frame. `body` borrows the frame.
pub struct AuthFrame<'a> {
    pub algorithm: u16,
    pub seq: u16,
    pub status: u16,
    pub body: &'a [u8],
}

/// Whether a management frame's addresses say it is from `bssid` to `us`:
/// receiver us, transmitter and BSSID the access point.
pub fn from_bss_to_us(frame: &[u8], us: &MacAddr, bssid: &MacAddr) -> bool {
    frame.len() >= MAC_HEADER_LEN
        && frame[4..10] == us[..]
        && frame[10..16] == bssid[..]
        && frame[16..22] == bssid[..]
}

/// Parse an Authentication frame from `bssid` to `us`. `None` for any other
/// frame, or one too short for its fixed fields.
pub fn parse_auth<'a>(frame: &'a [u8], us: &MacAddr, bssid: &MacAddr) -> Option<AuthFrame<'a>> {
    if frame.len() < MAC_HEADER_LEN + 6 || !from_bss_to_us(frame, us, bssid) {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    if fc_type(fc) != TYPE_MGMT || fc_subtype(fc) != SUBTYPE_AUTH {
        return None;
    }
    let f = &frame[MAC_HEADER_LEN..];
    Some(AuthFrame {
        algorithm: u16::from_le_bytes([f[0], f[1]]),
        seq: u16::from_le_bytes([f[2], f[3]]),
        status: u16::from_le_bytes([f[4], f[5]]),
        body: &f[6..],
    })
}

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
