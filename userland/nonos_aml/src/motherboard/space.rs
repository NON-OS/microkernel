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

//! The Word, DWord, QWord and Extended Address Space descriptors (ACPI
//! 6.5, 6.4.3.5.1 to 6.4.3.5.4) of the memory (0) and I/O (1) resource
//! types; a bus number range (2) is no address. The range is the minimum
//! and the length. A translation offset is a bridge's, and a motherboard
//! device is none: a descriptor that names one is left out rather than
//! reserved at an address it may not mean.

use super::fixed::{le, Reserved, Space};
use crate::platform::Item;

const WORD: u8 = 0x08;
const DWORD: u8 = 0x07;
const QWORD: u8 = 0x0A;
const EXTENDED: u8 = 0x0B;

/// Field width, where the minimum starts, and the body's least length.
fn layout(kind: u8) -> Option<(usize, usize, usize)> {
    match kind {
        WORD => Some((2, 5, 13)),
        DWORD => Some((4, 7, 23)),
        QWORD => Some((8, 11, 43)),
        // revision and a reserved byte before the granularity
        EXTENDED => Some((8, 13, 53)),
        _ => None,
    }
}

pub(super) fn address(i: &Item<'_>) -> Option<Reserved> {
    let (w, at, least) = layout(i.kind)?;
    let b = i.body;
    if b.len() < least {
        return None;
    }
    let space = match b[0] {
        0 => Space::Memory,
        1 => Space::Io,
        _ => return None,
    };
    // minimum, maximum, translation, length, one field after another
    let field = |n: usize| le(&b[at + n * w..at + (n + 1) * w]);
    let (base, translation, len) = (field(0), field(2), field(3));
    let fits = base.checked_add(len).is_some();
    (translation == 0 && len != 0 && fits).then_some(Reserved { space, base, len })
}
