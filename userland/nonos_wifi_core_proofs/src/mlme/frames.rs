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

//! The management frames the simulated access point answers with:
//! authentication, SAE, association responses, and a reader for the
//! elements of the association request it receives.

use crate::mlme::sim::{AP, STA};
use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};

// A management frame from `src` (also the BSSID) to `dst` with `body`.
pub fn mgmt(subtype: u8, src: [u8; 6], dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, subtype), dst, src, src, 0).unwrap();
    f.extend_from_slice(body);
    f
}

pub fn open_auth_ok(src: [u8; 6], dst: [u8; 6]) -> Vec<u8> {
    mgmt(11, src, dst, &[0, 0, 2, 0, 0, 0])
}

// An SAE Authentication frame from the AP to the station.
pub fn sae_from_ap(seq: u16, status: u16, body: &[u8]) -> Vec<u8> {
    let mut b = vec![3, 0];
    b.extend_from_slice(&seq.to_le_bytes());
    b.extend_from_slice(&status.to_le_bytes());
    b.extend_from_slice(body);
    mgmt(11, AP, STA, &b)
}

pub fn assoc_resp(status: u16) -> Vec<u8> {
    let mut body = vec![0x11, 0x04];
    body.extend_from_slice(&status.to_le_bytes());
    body.extend_from_slice(&0xC001u16.to_le_bytes());
    mgmt(1, AP, STA, &body)
}

// The elements of an association request, from after its fixed fields.
pub fn assoc_elements(req: &[u8]) -> Vec<(u8, Vec<u8>)> {
    let mut out = Vec::new();
    let mut off = MAC_HEADER_LEN + 4;
    while off + 2 <= req.len() {
        let len = req[off + 1] as usize;
        out.push((req[off], req[off + 2..off + 2 + len].to_vec()));
        off += 2 + len;
    }
    out
}
