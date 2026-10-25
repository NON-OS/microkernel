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

//! Callers speak 512-byte sectors; a driver speaks its disk's own blocks.
//!
//! The AHCI and virtio-blk drivers serve 512-byte sectors and so do most
//! NVMe namespaces, but an NVMe namespace may be formatted with 4096-byte
//! blocks, and the NVMe driver takes the address, the count and the
//! capacity in those. A 512-byte sector address sent as is lands eight
//! times too far in, and a capacity read as sectors is eight times too
//! small. This file turns a sector range into whole driver blocks, splits
//! it at the most one request may move, and writes a range that starts or
//! ends inside a block by reading that block first, so the sectors around
//! the written ones keep what they held.
//!
//! Nothing here touches the kernel: the requests go through [`Native`], so
//! the host tests drive this exact code against a disk in memory.

use alloc::vec;

use super::chunks::{place, read_blocks, write_blocks, Native};
use super::geometry::Geometry;

/// Read the sectors from `lba`, `out.len()` a whole non-zero number of
/// sectors; the caller has checked that.
pub fn read<N: Native>(n: &mut N, g: &Geometry, lba: u64, out: &mut [u8]) -> Result<(), N::Error> {
    let (first, skip) = place(g, lba);
    let block = g.lba_size() as usize;
    if skip == 0 && out.len().is_multiple_of(block) {
        return read_blocks(n, g, first, out);
    }
    let mut buf = vec![0u8; (skip + out.len()).div_ceil(block) * block];
    read_blocks(n, g, first, &mut buf)?;
    out.copy_from_slice(&buf[skip..skip + out.len()]);
    Ok(())
}

/// Write the sectors from `lba`. A block the range covers only in part is
/// read first and written back whole, with only the sectors asked for
/// changed.
pub fn write<N: Native>(n: &mut N, g: &Geometry, lba: u64, data: &[u8]) -> Result<(), N::Error> {
    let (first, skip) = place(g, lba);
    let block = g.lba_size() as usize;
    if skip == 0 && data.len().is_multiple_of(block) {
        return write_blocks(n, g, first, data);
    }
    let blocks = (skip + data.len()).div_ceil(block);
    let mut buf = vec![0u8; blocks * block];
    if skip != 0 {
        n.read_native(first, &mut buf[..block])?;
    }
    let end = skip + data.len();
    let last = blocks - 1;
    /* A range inside one block had that block read just above. */
    let tail_read = last != 0 || skip == 0;
    if !end.is_multiple_of(block) && tail_read {
        n.read_native(first + last as u64, &mut buf[last * block..])?;
    }
    buf[skip..end].copy_from_slice(data);
    write_blocks(n, g, first, &buf)
}
