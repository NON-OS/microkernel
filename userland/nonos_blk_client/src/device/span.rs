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

pub const SECTOR: usize = 512;

/// The smallest data buffer among the three drivers: sixty-four sectors.
pub const DATA_BYTES: u32 = 64 * SECTOR as u32;

/// The NVMe driver holds its controller at a 4 KiB memory page, and MDTS
/// counts in pages of that size.
const PAGE_SHIFT: u32 = 12;

/// A disk as its driver addresses it: the block size, and the most bytes
/// one request may move, a whole number of blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Geometry {
    lba_size: u32,
    max_bytes: u32,
}

impl Geometry {
    /// 512-byte blocks, sixty-four to a request: AHCI and virtio-blk.
    pub const SECTORS: Geometry = Geometry { lba_size: SECTOR as u32, max_bytes: DATA_BYTES };

    /// An NVMe namespace with blocks of `lba_size` bytes on a controller
    /// whose identify page gives `mdts`. The driver moves at most its data
    /// buffer per command, and less when MDTS says so; this mirrors the
    /// driver's own rule, so no request is one it refuses for its size.
    /// `None` for a block size that is not a power of two from 512 bytes to
    /// the buffer, or a transfer limit below one block.
    pub fn nvme(lba_size: u32, mdts: u8) -> Option<Geometry> {
        if lba_size < SECTOR as u32 || !lba_size.is_power_of_two() || lba_size > DATA_BYTES {
            return None;
        }
        let max = max_transfer_bytes(mdts);
        if max < lba_size {
            return None;
        }
        Some(Geometry { lba_size, max_bytes: max - max % lba_size })
    }

    pub fn lba_size(&self) -> u32 {
        self.lba_size
    }

    pub fn max_bytes(&self) -> u32 {
        self.max_bytes
    }

    /// Sectors in one block.
    pub fn per_block(&self) -> u64 {
        u64::from(self.lba_size) / SECTOR as u64
    }

    /// A capacity in blocks, as sectors. Saturated: a count that wrapped
    /// would describe a disk as a few bytes.
    pub fn sectors(&self, blocks: u64) -> u64 {
        blocks.saturating_mul(self.per_block())
    }
}

/// Bytes one NVMe command may move: the data buffer, or less when MDTS
/// says so. Zero means no limit, and so does a value too large to shift.
pub fn max_transfer_bytes(mdts: u8) -> u32 {
    let shift = u32::from(mdts) + PAGE_SHIFT;
    if mdts == 0 || shift >= u32::BITS {
        return DATA_BYTES;
    }
    (1u32 << shift).min(DATA_BYTES)
}

/// One driver request in the driver's own blocks. Slices are a whole,
/// non-zero number of blocks, no more than [`Geometry::max_bytes`].
pub trait Native {
    type Error;
    fn read_native(&mut self, lba: u64, out: &mut [u8]) -> Result<(), Self::Error>;
    fn write_native(&mut self, lba: u64, data: &[u8]) -> Result<(), Self::Error>;
}

/// Read the sectors from `lba`, `out.len()` a whole non-zero number of
/// sectors; the caller has checked that.
pub fn read<N: Native>(n: &mut N, g: &Geometry, lba: u64, out: &mut [u8]) -> Result<(), N::Error> {
    let (first, skip) = place(g, lba);
    let block = g.lba_size as usize;
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
    let block = g.lba_size as usize;
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

/// The block a sector is in, and the byte offset of the sector in it.
fn place(g: &Geometry, lba: u64) -> (u64, usize) {
    let per = g.per_block();
    (lba / per, (lba % per) as usize * SECTOR)
}

fn read_blocks<N: Native>(
    n: &mut N,
    g: &Geometry,
    first: u64,
    buf: &mut [u8],
) -> Result<(), N::Error> {
    let per_request = g.max_bytes as usize;
    let block = g.lba_size as usize;
    for (i, chunk) in buf.chunks_mut(per_request).enumerate() {
        n.read_native(first + (i * per_request / block) as u64, chunk)?;
    }
    Ok(())
}

fn write_blocks<N: Native>(
    n: &mut N,
    g: &Geometry,
    first: u64,
    buf: &[u8],
) -> Result<(), N::Error> {
    let per_request = g.max_bytes as usize;
    let block = g.lba_size as usize;
    for (i, chunk) in buf.chunks(per_request).enumerate() {
        n.write_native(first + (i * per_request / block) as u64, chunk)?;
    }
    Ok(())
}
