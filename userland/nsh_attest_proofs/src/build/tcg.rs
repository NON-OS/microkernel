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

//! TCG logs in the crypto-agile format, built field by field, with the PCR
//! values a TPM would hold after each event.

pub const SHA1: u16 = 0x0004;
pub const SHA256: u16 = 0x000B;
pub const EV_NO_ACTION: u32 = 3;
pub const EV_SEPARATOR: u32 = 4;
pub const EV_APP: u32 = 0x8000_0003;
pub const EV_ACTION: u32 = 0x8000_0007;

pub fn spec_body(banks: &[(u16, u16)], versions: [u8; 4]) -> Vec<u8> {
    let mut body = b"Spec ID Event03\0".to_vec();
    body.extend(0u32.to_le_bytes());
    body.extend(versions);
    body.extend((banks.len() as u32).to_le_bytes());
    for (a, s) in banks {
        body.extend(a.to_le_bytes());
        body.extend(s.to_le_bytes());
    }
    body.push(0);
    body
}

pub fn header_with(pcr: u32, kind: u32, digest: [u8; 20], body: &[u8]) -> Vec<u8> {
    let mut h = pcr.to_le_bytes().to_vec();
    h.extend(kind.to_le_bytes());
    h.extend(digest);
    h.extend((body.len() as u32).to_le_bytes());
    h.extend(body);
    h
}

pub fn header() -> Vec<u8> {
    header_with(0, EV_NO_ACTION, [0; 20], &spec_body(&[(SHA1, 20), (SHA256, 32)], [0, 2, 0, 2]))
}
