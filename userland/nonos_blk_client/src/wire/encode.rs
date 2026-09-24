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

//! Request header: magic, version, op, flags, pad, request id, payload
//! length, then the body. Twenty bytes, little-endian, the same for all
//! three drivers apart from the magic.

use alloc::vec::Vec;

use super::{HDR_LEN, VERSION};

pub fn encode_request(magic: u32, op: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HDR_LEN + body.len());
    out.extend_from_slice(&magic.to_le_bytes());
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(body);
    out
}

/// The twelve-byte read/write prefix: lba, then the sector count.
pub fn rw_header(lba: u64, sectors: u32) -> [u8; 12] {
    let mut h = [0u8; 12];
    h[..8].copy_from_slice(&lba.to_le_bytes());
    h[8..].copy_from_slice(&sectors.to_le_bytes());
    h
}
