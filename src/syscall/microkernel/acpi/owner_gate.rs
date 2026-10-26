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

//! Which ACPI numbers only the acpi owner's thread group may make (F-95).
//! The Admin bit alone let any Admin capsule map firmware memory, write
//! configuration space, report a critical battery or a pressed power
//! button, or add a table to the attested list. Every number the ACPI
//! dispatch routes is either here or in the shared list the proofs keep
//! (kernel_proofs, owner_gate_tests), which hold it so. Pure.

/// The capsule's own calls: tables, regions, ports, configuration, events,
/// and everything it publishes or asks about another process.
pub(crate) const OWNER_ONLY: [[u8; 4]; 25] = [
    *b"MTBL", *b"MRGN", *b"MRIO", *b"MRPC", *b"MAPW", *b"MAEV", *b"MASL", *b"MSMB", *b"MACP",
    *b"MAHO", *b"MPPF", *b"MPDP", *b"MAFO", *b"MAHE", *b"MAPD", *b"MAUP", *b"MAXP", *b"MAMR",
    *b"MAPX", *b"MAOG", *b"MALP", *b"MAFP", *b"MRSL", *b"MALD", *b"MATL",
];

pub(crate) fn owner_only(nr: u64) -> bool {
    OWNER_ONLY.iter().any(|t| u64::from(u32::from_le_bytes(*t)) == nr)
}
