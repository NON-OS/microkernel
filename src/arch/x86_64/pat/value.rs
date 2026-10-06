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

//! The page attribute table value with a write-combining entry, free of the
//! MSR so arch_paging_proofs checks it on the host.

/// IA32_PAT at reset, PA0 to PA7: WB, WT, UC-, UC, WB, WT, UC-, UC (SDM Vol.
/// 3A 12.12.4). The reset table has no write-combining entry at all.
pub const PAT_RESET: u64 = 0x0007_0406_0007_0406;

/// Memory type encodings of a PAT entry (SDM Vol. 3A 12.12.2).
pub const MT_UC: u8 = 0x00;
pub const MT_WC: u8 = 0x01;
pub const MT_WT: u8 = 0x04;
pub const MT_WB: u8 = 0x06;
pub const MT_UC_MINUS: u8 = 0x07;

/// The entry a 4 KiB page selects with PWT alone (PAT and PCD clear).
/// Linux's pat_bp_init puts write-combining in the same entry.
pub const WC_INDEX: u32 = 1;

/// `pat` with entry 1 made write-combining and every other entry kept, so a
/// page mapped write-back, UC- or UC keeps the type it had.
pub fn with_wc(pat: u64) -> u64 {
    let shift = WC_INDEX * 8;
    (pat & !(0xFF << shift)) | ((MT_WC as u64) << shift)
}

/// The PAT entry a 4 KiB page selects from its PAT, PCD and PWT bits.
pub fn index(pat_bit: bool, pcd: bool, pwt: bool) -> u32 {
    ((pat_bit as u32) << 2) | ((pcd as u32) << 1) | pwt as u32
}

/// The memory type held in entry `i` of `pat`.
pub fn entry(pat: u64, i: u32) -> u8 {
    ((pat >> (i * 8)) & 0x07) as u8
}
