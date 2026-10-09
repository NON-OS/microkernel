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

//! The block surface, for the kernel's client alone: capacity, whole
//! 512-byte sectors in and out on a device of any served block length, and
//! a flush. It arrives as pid 0; any other sender is refused, so the
//! medium is written only through the kernel's block layer.

use super::block_rw::{read_write, Rw};
use crate::disk::{sync_cache, Disk};
use crate::protocol::*;
use crate::scan::Medium;
use crate::server::respond;
use crate::span::{sectors, sectors_per_block, SECTOR_BYTES};
use crate::state::State;

pub fn handle(
    state: &mut State,
    medium: &Medium,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) {
    let status = match (sender, medium) {
        (0, Medium::Ready(disk)) => serve(state, disk, req, body, tx),
        (0, Medium::Scanning) => Err(E_AGAIN),
        (0, Medium::Absent) => Err(E_NODEV),
        _ => Err(E_ACCES),
    };
    /*
     * A write or a flush replies with no data: a payload of none is the
     * bare status word.
     */
    let _ = match status {
        Ok(n) => respond::payload(sender, req, n, tx),
        Err(e) => respond::status(sender, req, e, tx),
    };
}

/// Serve `req` on `disk`; the reply's data, if any, is already in `tx`.
fn serve(
    state: &mut State,
    disk: &Disk,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<usize, i32> {
    let out = &mut tx[HDR_LEN + STATUS_LEN..];
    match req.op {
        OP_BLK_CAPACITY if body.is_empty() => {
            // In the 512-byte sectors reads and writes take. A block length
            // that is not served is told as it is, and the kernel passes the
            // device over by name.
            let (count, len) = match sectors_per_block(disk.block_len) {
                Some(per) => (sectors(disk.blocks, per), SECTOR_BYTES),
                None => (disk.blocks, disk.block_len),
            };
            out[..8].copy_from_slice(&count.to_le_bytes());
            out[8..12].copy_from_slice(&len.to_le_bytes());
            Ok(12)
        }
        OP_BLK_FLUSH if body.is_empty() => sync_cache(disk, state).map(|_| 0),
        OP_BLK_READ => read_write(state, disk, Rw::Read(out), body),
        OP_BLK_WRITE => read_write(state, disk, Rw::Write, body).map(|_| 0),
        _ => Err(E_INVAL),
    }
}
