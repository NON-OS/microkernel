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

//! Which way a grant's data moves, as the device's IOMMU mapping enforces it.
//! Setting neither is a grant both sides use, such as a descriptor ring. Pure,
//! so the host proofs hold it.

use super::flags::{DMA_MAP_FROM_DEVICE, DMA_MAP_TO_DEVICE};

/// Both directions at once is not a third kind of grant: it is refused, so a
/// driver that meant one of them finds out at map time.
pub(super) const fn conflicting(flags: u32) -> bool {
    flags & DMA_MAP_TO_DEVICE != 0 && flags & DMA_MAP_FROM_DEVICE != 0
}

/// Whether the device may read the grant: not when it only receives into it.
pub(super) const fn device_reads(flags: u32) -> bool {
    flags & DMA_MAP_FROM_DEVICE == 0
}

/// Whether the device may write the grant: not when it only sends from it.
pub(super) const fn device_writes(flags: u32) -> bool {
    flags & DMA_MAP_TO_DEVICE == 0
}
