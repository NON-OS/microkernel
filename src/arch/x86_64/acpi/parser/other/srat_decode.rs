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

//! One SRAT entry (ACPI 6.5, 5.2.16), laid out as the packed structs in
//! `tables/srat_*.rs` declare: type and length, then per kind the
//! proximity domain, the processor or range or device, and flags whose bit
//! 0 says enabled. An entry shorter than its kind is skipped.

use super::srat_entry::{Handle, SratEntry};

fn u32_at(e: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([e[at], e[at + 1], e[at + 2], e[at + 3]])
}

fn u64_at(e: &[u8], at: usize) -> u64 {
    u64::from(u32_at(e, at)) | u64::from(u32_at(e, at + 4)) << 32
}

pub(super) fn entry(e: &[u8]) -> Option<SratEntry> {
    let on = |at: usize| u32_at(e, at) & 1 != 0;
    Some(match (e[0], e.len()) {
        (0, 16..) if on(4) => {
            // the domain's low byte, then its high three bytes after the SAPIC EID
            let domain = u32::from(e[2]) | u32::from_le_bytes([0, e[9], e[10], e[11]]);
            SratEntry::Apic { apic_id: u32::from(e[3]), domain }
        }
        (1, 40..) if on(28) => {
            let (flags, domain) = (u32_at(e, 28), u32_at(e, 2));
            let (base, len) = (u64_at(e, 8), u64_at(e, 16));
            let (hot_pluggable, non_volatile) = (flags & 2 != 0, flags & 4 != 0);
            SratEntry::Memory { domain, base, len, hot_pluggable, non_volatile }
        }
        (2, 24..) if on(12) => SratEntry::Apic { apic_id: u32_at(e, 8), domain: u32_at(e, 4) },
        (3, 18..) if on(10) => SratEntry::Gicc { uid: u32_at(e, 6), domain: u32_at(e, 2) },
        (5, 32..) if on(24) => SratEntry::Initiator { domain: u32_at(e, 4), handle: handle(e)? },
        _ => return None,
    })
}

/// The device handle: type 1 PCI (segment, then bus and devfn), type 0
/// ACPI (_HID, then _UID).
fn handle(e: &[u8]) -> Option<Handle> {
    let (segment, bus, devfn) = (u16::from_le_bytes([e[8], e[9]]), e[10], e[11]);
    match e[3] {
        0 => Some(Handle::Acpi { hid: e[8..16].try_into().ok()?, uid: u32_at(e, 16) }),
        1 => Some(Handle::Pci { segment, bus, devfn }),
        _ => None,
    }
}
