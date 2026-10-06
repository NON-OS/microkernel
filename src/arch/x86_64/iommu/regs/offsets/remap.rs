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

//! The interrupt remapping registers (VT-d 3.4, sections 5.1 and 11.4.4).

/// Interrupt Remapping Table Address: base in bits 63:12, EIME in bit 11,
/// and S in bits 3:0 for a table of 2^(S + 1) entries.
pub const IRTA: usize = 0x0B8;
pub const IRTA_EIME: u64 = 1 << 11;

/// One 4 KiB page of 16-byte entries.
pub const IRTE_ENTRIES: u16 = 256;
const IRTA_SIZE_256: u64 = 7;

/// GCMD bits: IRE (25) and CFI (23) persist; SIRTP (24) is a one-shot that
/// `gcmd_with` already drops from a GSTS copy.
pub const GCMD_IRE: u32 = 1 << 25;
pub const GCMD_SIRTP: u32 = 1 << 24;
pub const GCMD_CFI: u32 = 1 << 23;
pub const GSTS_IRES: u32 = 1 << 25;
pub const GSTS_IRTPS: u32 = 1 << 24;
pub const GSTS_CFIS: u32 = 1 << 23;

/// IRTA for a page-aligned table of `IRTE_ENTRIES`. `x2apic` sets EIME, and
/// with it every entry's destination is read as a 32-bit x2APIC id; without
/// it as an 8-bit xAPIC id in bits 47:40.
pub const fn irta_value(table_phys: u64, x2apic: bool) -> u64 {
    let eime = if x2apic { IRTA_EIME } else { 0 };
    (table_phys & !0xFFF) | eime | IRTA_SIZE_256
}

/// Entries a table named by an IRTA value holds.
pub const fn irta_entries(irta: u64) -> u32 {
    1 << ((irta & 0xF) + 1)
}
