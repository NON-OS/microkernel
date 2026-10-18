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

use super::*;

/// Management subtypes for leaving.
pub const SUBTYPE_DISASSOC: u8 = 10;

pub const SUBTYPE_DEAUTH: u8 = 12;

/// The broadcast address, which a deauthentication may be sent to.
pub(super) const BROADCAST: MacAddr = [0xFF; 6];

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
