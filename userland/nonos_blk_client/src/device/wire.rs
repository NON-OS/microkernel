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

//! What the block wire's GEOMETRY and IDENTIFY replies tell the installer:
//! how its disk is addressed, and what the part is called.

use super::geometry::Geometry;
use super::identity_parse::Identity;
use super::wire_identity_parse::{parse_wire_identity, OP_WIRE_IDENTIFY, WIRE_IDENTIFY_LEN};
use crate::driver::Driver;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

const OP_GEOMETRY: u16 = 2;
const GEOMETRY_LEN: usize = 20;
const MEDIA_PRESENT: u32 = 1 << 2;
const ENOMEDIUM: i32 = -123;

/// The block size and per-request ceiling from GEOMETRY. An empty slot is
/// the driver's ENOMEDIUM; a block size the client cannot address is
/// refused by name, as an NVMe one is.
pub fn geometry(port: u32) -> Result<Geometry, BlkError> {
    let magic = Driver::Wire.magic();
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + GEOMETRY_LEN];
    let (n, id) = call(port, magic, OP_GEOMETRY, &[], &mut rx)?;
    let body = decode_reply(&rx, n, magic, OP_GEOMETRY, id)?;
    let b = body.get(..GEOMETRY_LEN).ok_or(BlkError::BadLength)?;
    let u32_at = |at: usize| u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]);
    if u32_at(16) & MEDIA_PRESENT == 0 {
        return Err(BlkError::Status(ENOMEDIUM));
    }
    let lba_size = u32_at(8);
    Geometry::wire(lba_size, u32_at(12)).ok_or(BlkError::BlockSize(lba_size))
}

/// The part's name, or `None` when the driver has none to give (ENODEV) or
/// sent one the wire's rules refuse: the disk is listed by bus and size.
pub fn identity(port: u32) -> Option<Identity> {
    let magic = Driver::Wire.magic();
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + WIRE_IDENTIFY_LEN];
    let (n, id) = call(port, magic, OP_WIRE_IDENTIFY, &[], &mut rx).ok()?;
    let body = decode_reply(&rx, n, magic, OP_WIRE_IDENTIFY, id).ok()?;
    parse_wire_identity(body)
}
