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

//! Which grants need frames the kernel itself maps uncached.

use crate::hardware::broker::dma::flags::{DMA_MAP_COHERENT, DMA_MAP_WC};

/// An uncached grant (rings, descriptors) for a device that does not snoop,
/// where the architecture says a cacheable kernel alias would be a mismatch.
/// Its frames come from the uncached pool or not at all: anywhere else the
/// kernel's mapping of them is cacheable while the capsule's is not. A
/// device that snoops is left as it was.
pub(super) fn needs_uncached_frames(device_id: u64, flags: u32) -> bool {
    flags & (DMA_MAP_COHERENT | DMA_MAP_WC) != 0
        && crate::arch::uncached_grants_need_uncached_frames()
        && !crate::hardware::broker::table::dma_coherent(device_id)
}
