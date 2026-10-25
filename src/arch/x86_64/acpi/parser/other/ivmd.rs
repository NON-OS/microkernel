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

//! IVMD: memory firmware reserved for devices behind an AMD IOMMU (spec
//! 48882, 5.2.2.3), the ranges a device keeps one to one while unclaimed
//!, as RMRRs are on VT-d. Pure, over the IVRS bytes, so the host
//! proofs can feed it hostile tables.

const FIRST_BLOCK: usize = 48;
const IVMD_LEN: usize = 32;

pub use super::ivmd_block::Reserved;
use super::ivmd_block::{block, ALL, ONE, RANGE};

pub const MAX_RESERVED: usize = 16;

/// Each IVMD block's range, in table order, written to `out`; how many.
/// A block that is short, runs past the table, wraps, or names nothing is
/// skipped; a block length that runs past the table ends the walk.
pub fn reserved(table: &[u8], out: &mut [Reserved]) -> usize {
    let (mut found, mut at) = (0, FIRST_BLOCK);
    while at + 4 <= table.len() && found < out.len() {
        let kind = table[at];
        let length = u16::from_le_bytes([table[at + 2], table[at + 3]]) as usize;
        if length < 4 || at + length > table.len() {
            break;
        }
        if matches!(kind, ALL | ONE | RANGE) && length >= IVMD_LEN {
            if let Some(r) = block(kind, &table[at..at + IVMD_LEN]) {
                out[found] = r;
                found += 1;
            }
        }
        at += length;
    }
    found
}
