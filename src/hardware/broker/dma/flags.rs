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

//! The `MkDmaMap` flags a driver may pass, as the libc names them.

pub(super) const DMA_MAP_HIGH: u32 = 1 << 0;
/// The device takes only 32-bit DMA addresses: the frames must lie below
/// 4 GiB, or the map fails. Not with `DMA_MAP_HIGH`.
pub(super) const DMA_MAP_DMA32: u32 = 1 << 1;
/// A descriptor ring or anything else the device and the driver both write
/// while it runs: mapped uncached, so neither side needs a cache flush, as
/// Linux dma_alloc_coherent. Not with `DMA_MAP_WC`.
pub(super) const DMA_MAP_COHERENT: u32 = 1 << 2;
/// A buffer the driver fills whole before the doorbell: mapped
/// write-combining where the PAT has the entry, else uncached. The driver
/// fences (sfence) before it rings the device.
pub(super) const DMA_MAP_WC: u32 = 1 << 3;
