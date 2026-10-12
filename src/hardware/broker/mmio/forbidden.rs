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

//! Physical ranges no MMIO grant may cover. Pure, and proven on
//! the host (kernel_proofs driver_grants rows_k6).
//!
//! A BAR is whatever the device record says, and a record can name memory
//! that is not the device's: RAM, the kernel's own image, or an IOMMU's
//! registers, which would hand a capsule the machine. Unlike the MSI-X
//! tables, which only cut a window short, any overlap with these refuses
//! the grant whole.

/// Whether the pages `[start, end)` touch any `[lo, hi)` of `forbidden`.
/// An empty or inverted range touches nothing; a forbidden range with
/// `hi <= lo` is skipped.
pub(crate) fn touches(start: u64, end: u64, forbidden: &[(u64, u64)]) -> bool {
    start < end && forbidden.iter().any(|&(lo, hi)| lo < hi && start < hi && lo < end)
}

/// EFI memory types that are not RAM: reserved (0), MMIO (11), MMIO port
/// space (12). Every other type is memory some code or data may live in,
/// firmware's included, so a grant over it is refused.
pub(crate) const fn is_ram(memory_type: u32) -> bool {
    !matches!(memory_type, 0 | 11 | 12)
}

/// `[start, start + pages * 4096)`, or None when it overflows.
pub(crate) fn page_span(start: u64, pages: u64) -> Option<(u64, u64)> {
    pages.checked_mul(4096).and_then(|len| start.checked_add(len)).map(|end| (start, end))
}
