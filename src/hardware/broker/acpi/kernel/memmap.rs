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

//! The firmware memory map's word on a physical range, for the rule.

use super::super::rule::MemKind;
use crate::boot::handoff::{get_handoff, memory_type as t};

const PAGE: u64 = 4096;

/// Every kind the map gives some of [base, end), and Unmapped for any part
/// it leaves out. Entries that overlap can only hide a gap, which hides an
/// Unmapped and never adds a grant; with no map at all nothing is granted.
pub(in super::super) fn kinds(base: u64, end: u64, each: &mut dyn FnMut(MemKind)) {
    let Some(handoff) = get_handoff() else {
        each(MemKind::Other);
        return;
    };
    // SAFETY: the boot validated the handoff's map before publishing it, and
    // nothing writes it after.
    let entries = unsafe { handoff.mmap.entries() };
    let mut covered = 0u64;
    for e in entries {
        let hi = e.physical_start.saturating_add(e.page_count.saturating_mul(PAGE));
        let (lo, hi) = (e.physical_start.max(base), hi.min(end));
        if lo < hi {
            covered = covered.saturating_add(hi - lo);
            each(kind(e.memory_type));
        }
    }
    if covered < end - base {
        each(MemKind::Unmapped);
    }
}

/// UEFI 2.10 section 7.2, EFI_MEMORY_TYPE. Loader and boot services memory
/// holds the kernel and what the boot left it, so it counts as RAM too.
fn kind(memory_type: u32) -> MemKind {
    match memory_type {
        t::CONVENTIONAL
        | t::LOADER_CODE
        | t::LOADER_DATA
        | t::BOOT_SERVICES_CODE
        | t::BOOT_SERVICES_DATA => MemKind::Ram,
        t::RESERVED => MemKind::Reserved,
        t::ACPI_NVS => MemKind::AcpiNvs,
        t::ACPI_RECLAIM => MemKind::AcpiReclaim,
        t::MMIO | t::MMIO_PORT_SPACE => MemKind::Unmapped,
        _ => MemKind::Other,
    }
}
