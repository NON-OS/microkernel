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

//! AMD interrupt remapping's encodings (spec 48882, 2.2.5), pure (K4): a
//! device table entry's interrupt quadword and a 32-bit remapping entry,
//! in the bit positions the spec's entry formats give.
//! An interrupt from a device whose entry has no table is target aborted,
//! and NMI, INIT, ExtInt and LINT pass for none: no device writes a
//! message that reaches a CPU but through an entry the kernel wrote.

/// IV, with IntCtl 00b (abort) and no table: every interrupt refused.
pub const ABORT_ALL: u64 = 1 << 0;
const TABLE_LEN_512: u64 = 9 << 1;
const TABLE_MASK: u64 = 0x000F_FFFF_FFFF_FFC0;
const INTCTL_REMAP: u64 = 0b10 << 60;
const PASSES: u64 = 0b111 << 56 | 0b11 << 62;
/// Entries per table: one per vector, so an entry's index is its vector.
pub const ENTRIES: usize = 512;

/// IV with IntCtl 10b: the device's interrupts go through the 512 entries
/// at `table` (128-byte aligned), and only those.
pub const fn remapped(table: u64) -> u64 {
    ABORT_ALL | TABLE_LEN_512 | (table & TABLE_MASK) | INTCTL_REMAP
}

/// Whether an interrupt quadword lets anything through unremapped.
pub const fn passes_any(q: u64) -> bool {
    q & ABORT_ALL == 0 || q & PASSES != 0 || q & (0b11 << 60) == 0b01 << 60
}

const REMAP_EN: u32 = 1 << 0;

/// Fixed delivery, physical destination, `vector` to xAPIC id `dest`.
pub const fn irte(vector: u8, dest: u8) -> u32 {
    REMAP_EN | (dest as u32) << 8 | (vector as u32) << 16
}
