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

//! Writing a reply into the send buffer; each returns the reply's length.

use super::wire::{Request, DATA_AT, HDR_LEN, MAGIC, STATUS_LEN, VERSION};

/// The header and `status`, with `data_len` payload bytes the caller has
/// written (or will write) at `DATA_AT`.
pub fn reply(tx: &mut [u8], req: &Request, status: i32, data_len: usize) -> usize {
    let plen = (STATUS_LEN + data_len) as u32;
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&req.op.to_le_bytes());
    tx[8..10].copy_from_slice(&req.flags.to_le_bytes());
    tx[10..12].fill(0);
    tx[12..16].copy_from_slice(&req.request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&plen.to_le_bytes());
    tx[HDR_LEN..DATA_AT].copy_from_slice(&status.to_le_bytes());
    DATA_AT + data_len
}

/// A reply carrying only `status`.
pub fn status(tx: &mut [u8], req: &Request, status: i32) -> usize {
    reply(tx, req, status, 0)
}
