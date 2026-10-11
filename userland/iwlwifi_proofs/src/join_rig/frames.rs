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

//! Frames the access point sends, and how they reach the firmware model.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};

use super::*;
use crate::gen3_model::Out;

/// A beacon from the access point on channel 6 with these elements after
/// the SSID, the 802.11b/g rates (CCK basic) and the DS parameter.
pub fn beacon(elements: &[&[u8]]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, 8), [0xFF; 6], AP, AP, 0).unwrap();
    f.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&[0x64, 0, 0x31, 0x04]); // 100 TU; ESS, privacy, short preamble
    f.extend_from_slice(&[0, SSID.len() as u8]);
    f.extend_from_slice(SSID);
    f.extend_from_slice(&[1, 8, 0x82, 0x84, 0x8B, 0x96, 0x0C, 0x12, 0x18, 0x24]);
    f.extend_from_slice(&[3, 1, 6]);
    f.extend_from_slice(&[5, 4, 0, 2, 0, 0]); // TIM: DTIM period 2
    for e in elements {
        f.extend_from_slice(e);
    }
    f
}

/// An RX MPDU notification carrying `frame`, with the firmware's status.
pub fn rx(frame: &[u8], status: u32) -> Out {
    let mut p = vec![0u8; 64];
    p[0..2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    p[12..16].copy_from_slice(&(status | 0x3).to_le_bytes());
    p.extend_from_slice(frame);
    Out { cmd: 0xC1, group: 0, seq: 0x8000, payload: p }
}

pub(super) fn mgmt(subtype: u8, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, subtype), STA, AP, AP, 0).unwrap();
    f.extend_from_slice(body);
    f
}

/// A data frame from the access point to the station, in the clear.
pub fn from_ap(payload_with_snap: &[u8], seq: u16) -> Vec<u8> {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&STA);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&(seq << 4).to_le_bytes());
    f.extend_from_slice(payload_with_snap);
    f
}

pub fn eapol_from_ap(eapol: &[u8], seq: u16) -> Vec<u8> {
    let mut body = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x88, 0x8E];
    body.extend_from_slice(eapol);
    from_ap(&body, seq)
}
