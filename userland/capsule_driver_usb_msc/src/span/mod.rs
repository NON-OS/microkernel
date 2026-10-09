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

//! The kernel addresses every disk in 512-byte sectors; a USB device may
//! have logical blocks of 1, 2 or 4 KiB (a 4Kn drive in a USB-SATA case,
//! some large sticks). A request in sectors becomes the device blocks that
//! cover it. One that starts or ends inside a block is read whole and the
//! sectors asked for taken out of it, or for a write read, changed and
//! written back whole. Before this such a device was refused.

/// The kernel's sector.
pub const SECTOR_BYTES: u32 = 512;

/// Sectors in one logical block of `block_len` bytes: 1, 2, 4 or 8. None
/// for any other length, which is not served.
pub fn sectors_per_block(block_len: u32) -> Option<u32> {
    match block_len {
        512 | 1024 | 2048 | 4096 => Some(block_len / SECTOR_BYTES),
        _ => None,
    }
}

/// The device blocks a sector request covers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// The first device block.
    pub first: u64,
    /// How many device blocks.
    pub blocks: u32,
    /// Bytes of the first block before the first sector asked for.
    pub head: usize,
    /// The request starts and ends on block boundaries, so it moves as it
    /// is, with nothing read first.
    pub aligned: bool,
}

/// The span of `sectors` sectors from `lba`, with `per_block` sectors in a
/// block. None when the request is empty or its end overflows.
pub fn span(lba: u64, sectors: u32, per_block: u32) -> Option<Span> {
    if sectors == 0 || per_block == 0 {
        return None;
    }
    let per = per_block as u64;
    let end = lba.checked_add(sectors as u64)?;
    let first = lba / per;
    let last = end.div_ceil(per);
    Some(Span {
        first,
        blocks: u32::try_from(last - first).ok()?,
        head: ((lba % per) * SECTOR_BYTES as u64) as usize,
        aligned: lba.is_multiple_of(per) && end.is_multiple_of(per),
    })
}

/// The sectors a device of `blocks` blocks of `per_block` sectors holds.
pub fn sectors(blocks: u64, per_block: u32) -> u64 {
    blocks.saturating_mul(per_block as u64)
}

/// Whether blocks `first..first + blocks` need the 16-byte READ and WRITE:
/// past the 32-bit LBA or the 16-bit count of the 10-byte forms.
pub fn needs_cdb16(first: u64, blocks: u32) -> bool {
    first.saturating_add(blocks as u64) > 1u64 << 32 || blocks > u16::MAX as u32
}
