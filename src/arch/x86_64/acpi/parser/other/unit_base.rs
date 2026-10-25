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

//! Whether an IOMMU register base firmware gave can be mapped, pure (K11).
//! The kernel writes these registers: a base over RAM, past the address
//! width or over another unit would have it write memory that is not an
//! IOMMU's. A base refused here is never mapped, and its unit is left off.
//! kernel_proofs::unit_base.

/// The VT-d register page (iommu/unit/access.rs UNIT_WINDOW).
pub(crate) const DRHD_WINDOW: u64 = 4096;
/// An AMD IOMMU's register window (iommu/amd_vi/regs.rs WINDOW).
pub(crate) const AMD_WINDOW: u64 = 0x4000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BadBase {
    Zero,
    Unaligned,
    PastAddressWidth,
    OverRam,
    OverAnotherUnit,
}

/// `window` bytes at `base`, against the CPU's `phys_bits`, the RAM ranges
/// `[lo, hi)`, and the bases of units already kept, each `window` long.
pub(crate) fn check_base(
    base: u64,
    window: u64,
    phys_bits: u32,
    ram: &[(u64, u64)],
    kept: &[u64],
) -> Result<(), BadBase> {
    if base == 0 {
        return Err(BadBase::Zero);
    }
    if !base.is_multiple_of(4096) {
        return Err(BadBase::Unaligned);
    }
    let top = 1u64.checked_shl(phys_bits).unwrap_or(u64::MAX);
    let end = match base.checked_add(window) {
        Some(end) if end <= top => end,
        _ => return Err(BadBase::PastAddressWidth),
    };
    if ram.iter().any(|&(lo, hi)| lo < hi && base < hi && lo < end) {
        return Err(BadBase::OverRam);
    }
    if kept.iter().any(|&k| base < k.saturating_add(window) && k < end) {
        return Err(BadBase::OverAnotherUnit);
    }
    Ok(())
}
