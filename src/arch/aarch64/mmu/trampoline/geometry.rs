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

//! Where the EPD1 trampoline sits and the descriptors that map it: pure
//! arithmetic, so the proofs run the same code the kernel does.
//!
//! It owns top-level slot 508 outright: no user address reaches it (user
//! space ends below 256) and the kernel's layout leaves it empty in TTBR1.
//! Under it one table per level holds one entry, the last two: the code
//! page, then the page naming the kernel's vectors. Nothing else is mapped
//! there, so a user root shows exactly these two pages of the kernel.
//! `asm/epd1.h` holds the same address for the vectors.

/// The top-level slot, of 512, that holds the trampoline in every root.
pub(crate) const SLOT: usize = 508;
/// The trampoline's code page, TTBR0's view of slot 508's first page.
pub(crate) const BASE: u64 = (SLOT as u64) << 39;

const VALID_TABLE: u64 = 0b11;
const ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;
/// APTable[0] and UXNTable: nothing below is reachable or runs at EL0.
pub(crate) const NO_EL0: u64 = 1 << 61;
pub(crate) const NO_EL0_EXEC: u64 = 1 << 60;

/// A table descriptor that keeps EL0 out of everything under it, as the
/// leaves do too, so one wrong leaf cannot open a page to EL0 (Arm ARM,
/// DDI 0487: hierarchical permission controls).
pub(crate) const fn table(pa: u64) -> u64 {
    (pa & ADDR_MASK) | NO_EL0 | NO_EL0_EXEC | VALID_TABLE
}

/// A level-3 page descriptor with `bits` from `PageAttributes`.
pub(crate) const fn page(pa: u64, bits: u64) -> u64 {
    (pa & ADDR_MASK) | bits | VALID_TABLE
}

/// Every word the trampoline's tables hold: the root's slot, entry 0 of
/// each table below (`BASE` is its slot's first page), and the two leaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Chain {
    pub(crate) root: u64,
    pub(crate) l1: u64,
    pub(crate) l2: u64,
    pub(crate) code: u64,
    pub(crate) data: u64,
}

/// The chain through `tables` (L1, L2, L3), with the code page's and the
/// data page's attribute bits in `bits`.
pub(crate) const fn chain(tables: [u64; 3], code_pa: u64, data_pa: u64, bits: [u64; 2]) -> Chain {
    Chain {
        root: table(tables[0]),
        l1: table(tables[1]),
        l2: table(tables[2]),
        code: page(code_pa, bits[0]),
        data: page(data_pa, bits[1]),
    }
}
