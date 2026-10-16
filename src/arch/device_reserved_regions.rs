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

/// The requester's regions as `[base, end)`, and whether one that may name
/// it could not be resolved, so was left out.
pub(crate) fn device_reserved_regions(
    segment: u16,
    bdf: (u8, u8, u8),
) -> (heapless::Vec<(u64, u64), 16>, bool) {
    #[cfg(target_arch = "x86_64")]
    {
        let found = crate::arch::x86_64::iommu::unit::rmrr_of::rmrr_of(segment, bdf);
        (found.ranges, found.unresolved)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (segment, bdf);
        (heapless::Vec::new(), false)
    }
}
