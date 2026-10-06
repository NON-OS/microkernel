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

/// Physical span the identity domain covers, rounded up from what the frame
/// allocator reports. The floor exists because firmware often places
/// device-visible memory above the RAM the allocator was given, and a device
/// reaching past the map faults on a transfer nothing meant to block.
pub(super) fn identity_limit() -> u64 {
    const FLOOR: u64 = 4 * 1024 * 1024 * 1024;
    const GIB: u64 = 1024 * 1024 * 1024;
    /*
     * The top of the managed span, not the RAM total: RAM above a PCI hole
     * sits past the total, and a domain sized by it would miss that RAM.
     */
    let (_, top) = crate::memory::phys::allocator::phys_managed_range();
    let rounded = top.saturating_add(GIB - 1) & !(GIB - 1);
    if rounded < FLOOR {
        FLOOR
    } else {
        rounded
    }
}
