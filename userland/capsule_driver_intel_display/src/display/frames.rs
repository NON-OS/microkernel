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

//! The scanout surfaces' memory as the CPU writes it. The capsule's are
//! DMA grants mapped write-combining; the proofs' are the model's IOVA
//! pages, so a copy is checked by what the model's scanout then shows.

use nonos_gfx_wire::Errno;

/// A client's surface lent to the device for scanout: attached by this
/// driver and imported read only into the device's IOMMU domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lent {
    pub handle: u64,
    /// The IOVA the device reads it at, page 0 first, and its length.
    pub iova: u64,
    pub len: u64,
    /// The import to give back.
    pub id: u64,
}

pub trait Frames {
    /// Copy `src` to `offset` bytes into surface `frame` (0 or 1); false,
    /// with nothing written, when it would leave the surface.
    fn put(&mut self, frame: usize, offset: usize, src: &[u8]) -> bool;
    /// Order every copy before the flip that shows it (an sfence over
    /// write-combining memory).
    fn fence(&mut self);
    /// For a mode set: two new surfaces of `len` bytes become the ones
    /// `put` writes, the old ones kept; the new ones' first IOVAs, None
    /// when there is no room.
    fn renew(&mut self, len: usize) -> Option<[u64; 2]>;
    /// Keep the new surfaces and give the old back; or, `keep` false,
    /// give the new back and write the old ones again.
    fn settle(&mut self, keep: bool);
    /// The cursor's 64 x 64 ARGB image (16 KiB, the cursor plane's own
    /// layout) into its own memory, made on first use; its first IOVA.
    fn cursor_image(&mut self, img: &[u8]) -> Option<u64>;
    /// Row `row` of the band image (`band_plane::BYTES`, rows
    /// `band_plane::STRIDE` apart) from `src`, into its own memory made on
    /// first use; its first IOVA, None when there is no room or the row
    /// does not fit.
    fn band_row(&mut self, row: usize, src: &[u8]) -> Option<u64>;
    /// Lend the device client surface `handle`: EPERM when its owner did
    /// not let this driver read it, ENODEV when no IOMMU confines the
    /// device, ENOMEM when the domain has no room.
    fn lend(&mut self, handle: u64) -> Result<Lent, Errno>;
    /// The device can no longer read `lent` when this returns; then the
    /// attach goes.
    fn give_back(&mut self, lent: Lent);
}
