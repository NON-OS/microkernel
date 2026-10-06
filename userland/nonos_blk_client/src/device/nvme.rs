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

//! An NVMe namespace's geometry, asked of the driver. The driver serves
//! the namespace in its formatted blocks, 512 or 4096 bytes, and moves at
//! most what its buffer holds and MDTS allows per command; both come from
//! the identify pages it hands through.

use super::identity::{controller_page, MDTS_AT};
use super::span::Geometry;
use crate::driver::Driver;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

/// The driver's identify-namespace opcode and the payload it lays out:
/// nsid, size, capacity and utilisation in blocks, then the block size.
const OP_IDENTIFY_NAMESPACE: u16 = 4;
const NAMESPACE_LEN: usize = 36;
const LBA_SIZE_AT: usize = 28;

pub fn geometry(port: u32) -> Result<Geometry, BlkError> {
    let magic = Driver::Nvme.magic();
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + NAMESPACE_LEN];
    let (n, id) = call(port, magic, OP_IDENTIFY_NAMESPACE, &[], &mut rx)?;
    let body = decode_reply(&rx, n, magic, OP_IDENTIFY_NAMESPACE, id)?;
    if body.len() < NAMESPACE_LEN {
        return Err(BlkError::BadLength);
    }
    let mut w = [0u8; 4];
    w.copy_from_slice(&body[LBA_SIZE_AT..LBA_SIZE_AT + 4]);
    let lba_size = u32::from_le_bytes(w);
    let mdts = controller_page(port)?[MDTS_AT];
    Geometry::nvme(lba_size, mdts).ok_or(BlkError::BlockSize(lba_size))
}
