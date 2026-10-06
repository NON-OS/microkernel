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

//! IVRS: where the AMD IOMMUs are, so they can be taken back from firmware
//! that left one on, and which requester ids they cover, so the AMD-Vi driver
//! knows which devices it translates.

use spin::Mutex;

use super::super::state::TableRegistry;
use super::ivhd_scope::{spans, Span, MAX_SPANS};
use super::ivrs_walk::{iommu_bases, MAX_AMD_IOMMUS};
use crate::arch::x86_64::acpi::tables::SdtHeader;

const SIG_IVRS: u32 = u32::from_le_bytes(*b"IVRS");
/// Generous for any real IVRS; a longer one is read up to here.
const MAX_IVRS_BYTES: usize = 64 * 1024;

static BASES: Mutex<heapless::Vec<u64, MAX_AMD_IOMMUS>> = Mutex::new(heapless::Vec::new());
static SPANS: Mutex<heapless::Vec<Span, MAX_SPANS>> = Mutex::new(heapless::Vec::new());

/// Register bases of the AMD IOMMUs IVRS described.
pub fn amd_iommu_bases() -> heapless::Vec<u64, MAX_AMD_IOMMUS> {
    BASES.lock().clone()
}

/// The requester ids the IVHD device entries cover, in table order.
pub fn amd_iommu_spans() -> heapless::Vec<Span, MAX_SPANS> {
    SPANS.lock().clone()
}

pub fn parse_ivrs(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_IVRS) else {
        return;
    };
    let Some(addr) = super::super::phys::directmap(phys) else {
        return;
    };
    // SAFETY: eK@nonos.systems - the registry holds tables firmware published
    // and the directmap covers them; the header is read before the length it
    // carries is trusted, and the checksum is checked over that length.
    let bytes = unsafe {
        let header = core::ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = (header.length as usize).min(MAX_IVRS_BYTES);
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    let mut found = [0u64; MAX_AMD_IOMMUS];
    let n = iommu_bases(bytes, &mut found);
    *BASES.lock() = found[..n].iter().copied().collect();
    let mut covered = [Span { first: 0, last: 0, named: false }; MAX_SPANS];
    let n = spans(bytes, &mut covered);
    *SPANS.lock() = covered[..n].iter().copied().collect();
}
