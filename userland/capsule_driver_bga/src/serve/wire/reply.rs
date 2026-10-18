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

//! Replies, written into one buffer sized for the longest of them.

use super::header::{Request, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
use super::ops::PRIMARY_LEN;

pub const REPLY_LEN: usize = HDR_LEN + STATUS_LEN + PRIMARY_LEN;
pub type Reply = [u8; REPLY_LEN];

/// A reply that is only `status`; its length.
pub fn status(tx: &mut Reply, req: &Request, status: i32) -> usize {
    head(tx, req, STATUS_LEN as u32, status);
    HDR_LEN + STATUS_LEN
}

/// A reply of status 0 and `body`; its length.
pub fn payload(tx: &mut Reply, req: &Request, body: &[u8; PRIMARY_LEN]) -> usize {
    head(tx, req, (STATUS_LEN + PRIMARY_LEN) as u32, 0);
    tx[HDR_LEN + STATUS_LEN..].copy_from_slice(body);
    REPLY_LEN
}

fn head(tx: &mut Reply, req: &Request, payload_len: u32, status: i32) {
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&req.op.to_le_bytes());
    tx[8..10].copy_from_slice(&req.flags.to_le_bytes());
    tx[10..12].fill(0);
    tx[12..16].copy_from_slice(&req.request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&payload_len.to_le_bytes());
    tx[HDR_LEN..HDR_LEN + STATUS_LEN].copy_from_slice(&status.to_le_bytes());
}
