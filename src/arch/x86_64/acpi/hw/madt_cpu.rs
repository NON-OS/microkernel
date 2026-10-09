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

//! Which MADT processor entries name a processor the kernel may use.
//!
//! The rules are Linux's (`arch/x86/kernel/acpi/boot.c`):
//!
//! - Flags bit 0 is Enabled. Bit 1, Online Capable, exists only from MADT
//!   revision 5 (ACPI 6.3); before that it is reserved and must be ignored,
//!   so a disabled processor in an older MADT is not usable
//!   (`acpi_is_processor_usable`).
//! - A Local APIC entry (type 0) with APIC ID 0xFF is a reserved id, not a
//!   processor (`acpi_parse_lapic`). Likewise an x2APIC entry (type 9) with
//!   ID 0xFFFFFFFF.
//! - The spec reserves x2APIC entries for IDs of 255 and above. Firmware that
//!   lists every processor twice, once as type 0 and once as type 9 with the
//!   same small ID, is common; when any type 0 processor exists, type 9
//!   entries with an ID below 255 are ignored (`acpi_parse_x2apic`,
//!   `has_lapic_cpus`). x2APIC entries with IDs of 255 and above are always
//!   kept: those processors can only be addressed in x2APIC mode.
//! - A second entry for an APIC ID already seen is dropped.

pub const MADT_FLAG_ENABLED: u32 = 1 << 0;
pub const MADT_FLAG_ONLINE_CAPABLE: u32 = 1 << 1;

/// MADT revision that defines the Online Capable bit.
pub const MADT_REV_ONLINE_CAPABLE: u8 = 5;

pub const XAPIC_ID_INVALID: u32 = 0xFF;
pub const X2APIC_ID_INVALID: u32 = 0xFFFF_FFFF;

/// Whether a processor entry describes something that is or can be brought
/// online.
pub fn processor_usable(flags: u32, madt_revision: u8) -> bool {
    if flags & MADT_FLAG_ENABLED != 0 {
        return true;
    }
    madt_revision >= MADT_REV_ONLINE_CAPABLE && flags & MADT_FLAG_ONLINE_CAPABLE != 0
}

/// Whether a usable entry should be recorded, given whether the table holds
/// any usable type 0 processor at all.
pub fn keep_entry(apic_id: u32, is_x2apic: bool, table_has_xapic_cpus: bool) -> bool {
    if is_x2apic {
        if apic_id == X2APIC_ID_INVALID {
            return false;
        }
        !(table_has_xapic_cpus && apic_id < XAPIC_ID_INVALID)
    } else {
        apic_id != XAPIC_ID_INVALID
    }
}

/// True when any processor's APIC ID does not fit the 8-bit xAPIC
/// destination field, which means the system has to run the local APICs in
/// x2APIC mode to reach every processor.
pub fn needs_x2apic<I: IntoIterator<Item = u32>>(apic_ids: I) -> bool {
    apic_ids.into_iter().any(|id| id >= XAPIC_ID_INVALID)
}
