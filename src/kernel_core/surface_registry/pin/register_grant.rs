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

//! The DMA grant a surface is registered over, if any (L35).

use crate::kernel_core::surface_registry::types::SurfaceDescriptor;

/*
 * A range in a broker window is refused unless it is wholly one DMA grant of
 * the caller's own process: a GPU driver's scanout. None for any other
 * range, which then registers as the caller's memory or not at all.
 */
pub(super) fn own_grant(pid: u32, desc: &SurfaceDescriptor, device: bool) -> Option<(u64, u64)> {
    if !device {
        return None;
    }
    let holder = crate::hardware::broker::holder_of(pid);
    crate::hardware::broker::dma_grant_run_covering(holder, desc.base_va, desc.byte_len)
}
