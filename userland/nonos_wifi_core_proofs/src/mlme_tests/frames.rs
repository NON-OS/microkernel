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

//! The access point's beacon and authentication frames.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};

use super::base::{AP, SSID, STA};

// A beacon from `bssid` carrying the given elements after SSID and DS params.
pub(super) fn beacon(bssid: [u8; 6], elements: &[&[u8]]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, 8), [0xFF; 6], bssid, bssid, 0).unwrap();
    f.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    f.extend_from_slice(&[0, SSID.len() as u8]);
    f.extend_from_slice(SSID);
    f.extend_from_slice(&[3, 1, 6]);
    for e in elements {
        f.extend_from_slice(e);
    }
    f
}

pub(super) fn rsne(akms: &[u8], caps: u16, group: u8) -> Vec<u8> {
    let mut b = vec![1, 0, 0x00, 0x0f, 0xac, group, 1, 0, 0x00, 0x0f, 0xac, 4];
    b.extend_from_slice(&(akms.len() as u16).to_le_bytes());
    for a in akms {
        b.extend_from_slice(&[0x00, 0x0f, 0xac, *a]);
    }
    b.extend_from_slice(&caps.to_le_bytes());
    let mut e = vec![48, b.len() as u8];
    e.extend_from_slice(&b);
    e
}

// A management frame from `src` (also the BSSID) to `dst` with `body`.
pub(super) fn mgmt(subtype: u8, src: [u8; 6], dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, subtype), dst, src, src, 0).unwrap();
    f.extend_from_slice(body);
    f
}

pub(super) fn open_auth_ok(src: [u8; 6], dst: [u8; 6]) -> Vec<u8> {
    mgmt(11, src, dst, &[0, 0, 2, 0, 0, 0])
}

// An SAE Authentication frame from the AP to the station.
pub(super) fn sae_from_ap(seq: u16, status: u16, body: &[u8]) -> Vec<u8> {
    let mut b = vec![3, 0];
    b.extend_from_slice(&seq.to_le_bytes());
    b.extend_from_slice(&status.to_le_bytes());
    b.extend_from_slice(body);
    mgmt(11, AP, STA, &b)
}
