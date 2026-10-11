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

//! The access point's association response, and the elements the station sends.

use super::base::{AP, STA};
use super::frames::mgmt;
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;

pub(super) fn assoc_resp(status: u16) -> Vec<u8> {
    let mut body = vec![0x11, 0x04];
    body.extend_from_slice(&status.to_le_bytes());
    body.extend_from_slice(&0xC001u16.to_le_bytes());
    mgmt(1, AP, STA, &body)
}

// The elements of an association request, from after its fixed fields.
pub(super) fn assoc_elements(req: &[u8]) -> Vec<(u8, Vec<u8>)> {
    let mut out = Vec::new();
    let mut off = MAC_HEADER_LEN + 4;
    while off + 2 <= req.len() {
        let len = req[off + 1] as usize;
        out.push((req[off], req[off + 2..off + 2 + len].to_vec()));
        off += 2 + len;
    }
    out
}
