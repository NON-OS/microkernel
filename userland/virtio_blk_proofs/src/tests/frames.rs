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

//! Request frames for the parser tests, and the generator their sweeps
//! draw from.

use crate::protocol::{Request, RW_HEADER_LEN};

pub fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

pub fn req(payload_len: u32) -> Request {
    Request { op: 3, flags: 0, request_id: 1, payload_len }
}

/// The twelve-byte read/write prefix, then `data` bytes of zeros.
pub fn rw_body(lba: u64, nsectors: u32, data: usize) -> Vec<u8> {
    let mut b = vec![0u8; RW_HEADER_LEN + data];
    b[0..8].copy_from_slice(&lba.to_le_bytes());
    b[8..12].copy_from_slice(&nsectors.to_le_bytes());
    b
}
