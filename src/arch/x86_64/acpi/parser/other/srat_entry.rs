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

//! SRAT's entries from the table's bytes, with no pointer read: the one
//! SRAT decoder (`srat_decode` reads each entry). The x86 parser applies
//! what it yields; the aarch64 tables and `kernel_proofs` mount both files
//! by path, side by side. Disabled entries and kinds not listed here are
//! skipped; a short or overrunning entry ends the walk with what came
//! before.

use super::srat_decode::entry;
use alloc::vec::Vec;

/// SDT header, table revision, eight reserved bytes.
pub const ENTRIES_AT: usize = 48;
/// Entries kept; a four-socket server lists a few hundred.
pub const MAX_ENTRIES: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
    Pci { segment: u16, bus: u8, devfn: u8 },
    Acpi { hid: [u8; 8], uid: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SratEntry {
    /// Processor Local APIC (0) or x2APIC (2) affinity.
    Apic {
        apic_id: u32,
        domain: u32,
    },
    Memory {
        domain: u32,
        base: u64,
        len: u64,
        hot_pluggable: bool,
        non_volatile: bool,
    },
    /// GICC affinity: the processor by its ACPI processor UID.
    Gicc {
        uid: u32,
        domain: u32,
    },
    /// Generic Initiator affinity: a device that starts memory accesses.
    Initiator {
        domain: u32,
        handle: Handle,
    },
}

pub fn entries(table: &[u8]) -> Vec<SratEntry> {
    let (mut out, mut at) = (Vec::new(), ENTRIES_AT);
    while let Some(&len) = table.get(at + 1) {
        let len = usize::from(len);
        let Some(e) = table.get(at..at + len).filter(|_| len >= 2) else { break };
        if out.len() < MAX_ENTRIES {
            out.extend(entry(e));
        }
        at += len;
    }
    out
}
