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

//! The driver request `span` sends, where a sector falls in the disk's
//! blocks, and a run of whole blocks cut into requests no larger than one
//! may move.

use super::geometry::Geometry;
use super::transfer::SECTOR;

/// One driver request in the driver's own blocks. Slices are a whole,
/// non-zero number of blocks, no more than [`Geometry::max_bytes`].
pub trait Native {
    type Error;
    fn read_native(&mut self, lba: u64, out: &mut [u8]) -> Result<(), Self::Error>;
    fn write_native(&mut self, lba: u64, data: &[u8]) -> Result<(), Self::Error>;
}

/// The block a sector is in, and the byte offset of the sector in it.
pub(super) fn place(g: &Geometry, lba: u64) -> (u64, usize) {
    let per = g.per_block();
    (lba / per, (lba % per) as usize * SECTOR)
}

pub(super) fn read_blocks<N: Native>(
    n: &mut N,
    g: &Geometry,
    first: u64,
    buf: &mut [u8],
) -> Result<(), N::Error> {
    let per_request = g.max_bytes() as usize;
    let block = g.lba_size() as usize;
    for (i, chunk) in buf.chunks_mut(per_request).enumerate() {
        n.read_native(first + (i * per_request / block) as u64, chunk)?;
    }
    Ok(())
}

pub(super) fn write_blocks<N: Native>(
    n: &mut N,
    g: &Geometry,
    first: u64,
    buf: &[u8],
) -> Result<(), N::Error> {
    let per_request = g.max_bytes() as usize;
    let block = g.lba_size() as usize;
    for (i, chunk) in buf.chunks(per_request).enumerate() {
        n.write_native(first + (i * per_request / block) as u64, chunk)?;
    }
    Ok(())
}
