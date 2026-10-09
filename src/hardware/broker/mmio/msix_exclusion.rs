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

//! The MSI-X tables and pending-bit arrays a `MkMmioMap` request must not
//! put into a capsule address space, as physical ranges. The kernel programs
//! both regions on the capsule's behalf through the MSI-X bind path and
//! `MkPciConfigWrite`; exposing them via mmap would let a capsule
//! short-circuit the allowlist.
//!
//! Every device's regions are listed, not only the claimed device's: a
//! sub-page BAR is mapped by the page (`window`), and the rest of that page
//! may belong to another function. `window::window` then cuts the mapping
//! short at the page below the first region it would reach, so a device
//! whose registers share a BAR with its MSI-X table (e.g. xHCI) still maps
//! everything up to the table, and a request that starts in a protected
//! page is refused.

extern crate alloc;

use alloc::vec::Vec;

use crate::drivers::pci::constants::MSIX_ENTRY_SIZE;
use crate::drivers::pci::types::{MsixInfo, PciBar};
use crate::hardware::broker::pci_index;

/// The physical `[start, end)` of every MSI-X table and PBA the kernel knows.
pub fn protected_regions() -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for h in pci_index::all() {
        let Some(m) = h.msix else { continue };
        if let Some(r) = locate(&h.bars, m.table_bar, table_region(&m)) {
            out.push(r);
        }
        if let Some(r) = locate(&h.bars, m.pba_bar, pba_region(&m)) {
            out.push(r);
        }
    }
    out
}

/// A BAR-relative region placed at its BAR's physical base. A region whose
/// BAR is absent or not memory has no address to protect.
fn locate(bars: &[PciBar; 6], bar: u8, region: (u64, u64)) -> Option<(u64, u64)> {
    let b = bars.get(bar as usize)?;
    if !b.is_memory() {
        return None;
    }
    let base = b.address()?.as_u64();
    Some((base.checked_add(region.0)?, base.checked_add(region.1)?))
}

fn table_region(m: &MsixInfo) -> (u64, u64) {
    let start = m.table_offset as u64;
    let entries = (m.table_size as u64) + 1;
    let bytes = entries * MSIX_ENTRY_SIZE as u64;
    (start, start + bytes)
}

fn pba_region(m: &MsixInfo) -> (u64, u64) {
    let start = m.pba_offset as u64;
    let entries = (m.table_size as u64) + 1;
    let qwords = (entries + 63) / 64;
    let bytes = qwords * 8;
    (start, start + bytes)
}
