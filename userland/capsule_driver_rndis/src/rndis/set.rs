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

//! REMOTE_NDIS_SET_MSG (Remote NDIS 1.0, 2.2.6), as the station address
//! and the packet filter are set: 28 bytes of header, the value right
//! after, its offset counted from RequestID.

use alloc::vec;
use alloc::vec::Vec;

use super::message::{put32, MSG_SET};

/// The header's bytes, before the value.
const SET_LEN: usize = 28;

pub fn set_msg(oid: u32, value: &[u8]) -> Vec<u8> {
    let mut m = vec![0u8; SET_LEN + value.len()];
    let (len, size) = (m.len() as u32, value.len() as u32);
    for (at, v) in [(0, MSG_SET), (4, len), (12, oid), (16, size), (20, 20)] {
        put32(&mut m, at, v);
    }
    m[SET_LEN..].copy_from_slice(value);
    m
}
