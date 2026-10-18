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

//! Stream table entries and the level 1 descriptors above them.

pub type Ste = [u64; 8];

const VALID: u64 = 1;
const CONFIG_ABORT: u64 = 0b000 << 1;
const CONFIG_S1_TRANSLATE: u64 = 0b101 << 1;
/// Bits 51:6 of a table or CD address.
const PTR_MASK: u64 = ((1 << 52) - 1) & !0x3f;

/// Inner and outer write-back, read-allocate, inner shareable: how the unit
/// fetches CDs, matching how the kernel writes them.
const S1_CACHEABLE: u64 = 0b01 << 2 | 0b01 << 4 | 0b11 << 6;

/// The state every stream starts in and returns to: valid, so the unit does
/// not raise a bad-STE event, and every transaction aborted.
pub const fn abort() -> Ste {
    [VALID | CONFIG_ABORT, 0, 0, 0, 0, 0, 0, 0]
}

/// Translate through stage 1 with the single CD at `cd_phys`, for
/// non-secure EL1 (STRW 0). Word 0 holds the valid bit, so it is the word
/// written last.
pub const fn stage1(cd_phys: u64) -> Ste {
    [VALID | CONFIG_S1_TRANSLATE | (cd_phys & PTR_MASK), S1_CACHEABLE, 0, 0, 0, 0, 0, 0]
}

/// Span of a level 2 table holding `1 << split` STEs.
const fn span(split: u8) -> u64 {
    split as u64 + 1
}

/// Level 1 descriptor for a level 2 table of `1 << split` STEs at `l2_phys`.
/// Zero, the state of an untouched level 1 table, has span 0: the unit
/// treats those streams as out of range and aborts them.
pub const fn l1_descriptor(l2_phys: u64, split: u8) -> u64 {
    span(split) | (l2_phys & PTR_MASK)
}
