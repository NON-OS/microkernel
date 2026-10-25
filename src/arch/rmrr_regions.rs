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

//! The reserved regions a PCI requester keeps doing DMA to, by the platform's
//! own tables (L60): the RMRRs DMAR lists on x86_64. Elsewhere none are read
//! yet; AMD-Vi's IVMD and the IORT's reserved ranges are rows of their own.
//! The device tree's ranges on aarch64 are not these: they take the
//! all-or-nothing path in `reserved_regions` (C-40).

/// The requester's regions as `[base, end)`, and the regions left out: one
/// that may name it could not be resolved, or one names other devices too.
pub(crate) struct Reserved {
    pub ranges: heapless::Vec<(u64, u64), 16>,
    pub unresolved: bool,
    pub shared: bool,
}

pub(crate) fn device_rmrr_regions(segment: u16, bdf: (u8, u8, u8)) -> Reserved {
    #[cfg(all(target_arch = "x86_64", feature = "nonos-arch-iommu"))]
    {
        let found = crate::arch::x86_64::iommu::unit::rmrr_of::rmrr_of(segment, bdf);
        Reserved { ranges: found.ranges, unresolved: found.unresolved, shared: found.shared }
    }
    #[cfg(not(all(target_arch = "x86_64", feature = "nonos-arch-iommu")))]
    {
        let _ = (segment, bdf);
        Reserved { ranges: heapless::Vec::new(), unresolved: false, shared: false }
    }
}
