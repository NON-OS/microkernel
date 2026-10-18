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

//! The memory windows a controller's `_CRS` names: the 32-bit Fixed Memory
//! Range descriptor (ACPI 6.5, 6.4.3.4) and the DWord and QWord Address
//! Space descriptors of the memory type (6.4.3.5.2, 6.4.3.5.1).

use crate::legacy::Range;
use crate::platform::{items, Item};
use alloc::vec::Vec;

const MEMORY32_FIXED: u8 = 0x06;
const DWORD: u8 = 0x07;
const QWORD: u8 = 0x0A;
const MEMORY_TYPE: u8 = 0;

fn le(b: &[u8]) -> u64 {
    b.iter().rev().fold(0, |a, x| a << 8 | u64::from(*x))
}

/// Base and length, after the translation offset, which a device that is
/// not a bridge must list as zero (6.4.3.5): a window that says otherwise
/// is left out rather than placed where it might not be.
fn window(i: &Item<'_>) -> Option<Range> {
    let b = i.body;
    let (base, translation, len) = match i.kind {
        MEMORY32_FIXED if b.len() >= 9 => (le(&b[1..5]), 0, le(&b[5..9])),
        DWORD if b.len() >= 23 && b[0] == MEMORY_TYPE => {
            (le(&b[7..11]), le(&b[15..19]), le(&b[19..23]))
        }
        QWORD if b.len() >= 43 && b[0] == MEMORY_TYPE => {
            (le(&b[11..19]), le(&b[27..35]), le(&b[35..43]))
        }
        _ => return None,
    };
    (translation == 0 && len != 0 && base.checked_add(len).is_some()).then_some(Range { base, len })
}

/// In the order the template gives them, which the driver keeps: which
/// window holds which registers is the controller's to say, not ACPI's.
pub fn windows(t: &[u8]) -> Vec<Range> {
    items(t).iter().filter(|i| i.large).filter_map(window).collect()
}
