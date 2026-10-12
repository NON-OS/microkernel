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

//! A request, framed.

use alloc::vec::Vec;

use super::header::{HDR_LEN, MAGIC, VERSION};
use crate::bytes::len_u32;

/// The header for `op` and `request_id`, then `body`; None for a body whose
/// length does not fit its u32.
pub fn request(op: u16, request_id: u32, body: &[u8]) -> Option<Vec<u8>> {
    let len = len_u32(body.len())?;
    let mut tx = Vec::with_capacity(body.len().saturating_add(HDR_LEN));
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&len.to_le_bytes());
    tx.extend_from_slice(body);
    Some(tx)
}
