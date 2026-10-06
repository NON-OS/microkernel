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

//! The VMD domains brought up this boot. Set once, by the first enumeration.

extern crate alloc;

use alloc::vec::Vec;
use spin::Once;

use super::super::types::PciAddress;

/// Segment numbers handed to VMD domains start here, well clear of any real
/// segment a platform could report.
pub(super) const SEGMENT_BASE: u16 = 0x1000;

#[derive(Clone, Copy, Debug)]
pub(super) struct VmdDomain {
    pub segment: u16,
    /// The VMD endpoint on segment 0. Its requester id carries the
    /// children's DMA.
    pub vmd: PciAddress,
    /// Kernel virtual address of the mapped CFGBAR.
    pub cfg_va: u64,
    pub bus_start: u8,
    pub bus_count: u16,
}

pub(super) static DOMAINS: Once<Vec<VmdDomain>> = Once::new();

pub(super) fn find(segment: u16) -> Option<VmdDomain> {
    DOMAINS.get()?.iter().find(|d| d.segment == segment).copied()
}

/// How many VMD domains came up.
pub fn domain_count() -> usize {
    DOMAINS.get().map(|d| d.len()).unwrap_or(0)
}

/// The requester id a function's DMA arrives under. A function behind a VMD
/// is seen by the IOMMU as the VMD itself; everything else as itself.
pub fn dma_requester(address: PciAddress) -> PciAddress {
    if address.segment == 0 {
        return address;
    }
    find(address.segment).map(|d| d.vmd).unwrap_or(address)
}
