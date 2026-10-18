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

use core::ptr;

use super::super::state::TableRegistry;
use super::entries::*;
use super::walk::structures;
use super::x2apic::*;
use crate::arch::x86_64::acpi::hw::madt_cpu::{keep_entry, needs_x2apic, processor_usable};
use crate::arch::x86_64::acpi::tables::madt::*;
use crate::arch::x86_64::acpi::tables::{MAX_TABLE_BYTES, SIG_MADT};

pub fn parse_madt(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_MADT) else { return };
    let Some(addr) = super::super::phys::directmap(phys) else { return };
    // SAFETY: the directmap covers what firmware published; the fixed part
    // is read first, and the structures only through `walk`, which keeps
    // every one inside the length the header bounds.
    let (madt, table) = unsafe {
        let madt = ptr::read_volatile(addr as *const Madt);
        let len = u64::from(madt.header.length).min(MAX_TABLE_BYTES) as usize;
        (madt, core::slice::from_raw_parts(addr as *const u8, len))
    };
    registry.data.lapic_address = madt.local_apic_address as u64;
    registry.data.has_legacy_pics = madt.has_legacy_pics();
    let ctx = CpuContext {
        revision: madt.header.revision,
        has_xapic_cpus: table_has_xapic_cpus(table, madt.header.revision),
    };
    for s in structures(table) {
        let (at, len) = (s.as_ptr() as u64, s.len() as u8);
        match s[0] {
            0 => parse_local_apic(registry, at, len, ctx),
            1 => parse_ioapic(registry, at, len),
            2 => parse_interrupt_override(registry, at, len),
            4 => parse_local_apic_nmi(registry, at, len),
            5 => parse_lapic_override(registry, at, len),
            9 => parse_x2apic(registry, at, len, ctx),
            10 => parse_x2apic_nmi(registry, at, len),
            _ => {}
        }
    }

    // A processor whose APIC ID will not fit the 8-bit xAPIC destination field
    // is unreachable until the local APICs run in x2APIC mode; say so once the
    // whole table is recorded so a boot that leaves xAPIC on names the gap.
    if needs_x2apic(registry.data.processors.iter().map(|p| p.apic_id)) {
        crate::log::info!("[MADT] processor APIC ID exceeds xAPIC range; x2APIC required");
    }
}

/// Pass one: does any usable processor appear as a type 0 Local APIC? It
/// decides whether small-ID x2APIC entries are duplicates (`hw::madt_cpu`),
/// and entries come in no fixed order, so it is known before any is kept.
fn table_has_xapic_cpus(table: &[u8], revision: u8) -> bool {
    // type 0: processor uid at 2, APIC id at 3, flags u32 at 4
    structures(table).filter(|s| s[0] == 0 && s.len() >= 8).any(|s| {
        let flags = u32::from_le_bytes([s[4], s[5], s[6], s[7]]);
        processor_usable(flags, revision) && keep_entry(u32::from(s[3]), false, false)
    })
}
