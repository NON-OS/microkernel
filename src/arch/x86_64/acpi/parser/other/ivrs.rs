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
use super::ivrs_walk::{walk, MAX_AMD_IOMMUS};

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
    let Some((addr, header)) = super::super::phys::mapped_table(phys) else {
        return;
    };
    // SAFETY: eK@nonos.systems - the registry holds tables firmware published;
    // mapped_table found its whole declared length in the direct map, and the
    // checksum is checked over that length.
    let bytes = unsafe {
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = (header.length as usize).min(MAX_IVRS_BYTES);
        super::ivrs_left::note(len < header.length as usize);
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    let mut found = [0u64; MAX_AMD_IOMMUS];
    let read = walk(bytes, &mut found);
    super::ivrs_left::note(!read.complete);
    *BASES.lock() = super::ivrs_left::admitted(&found[..read.found]);
    let mut covered = [Span { first: 0, last: 0, named: false }; MAX_SPANS];
    let n = spans(bytes, &mut covered);
    *SPANS.lock() = covered[..n].iter().copied().collect();
    super::ivhd_alias_store::record_aliases(bytes);
    super::ivmd_table::record(bytes);
    super::ivhd_special_table::record(bytes);
}
