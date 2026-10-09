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

//! The entry array: one entry per partition in disk order, the rest zero.
//!
//! ```text
//!   0..16 type   16..32 unique   32..40 first LBA   40..48 last LBA
//!   48..56 attributes            56..128 name, UTF-16LE, 36 units
//! ```

use alloc::vec::Vec;

use super::shape::{ENTRY_COUNT, ENTRY_SIZE};
use crate::guid::Guid;
use crate::layout::{Layout, Region};

pub fn build_array(layout: &Layout, unique: &[Guid; 4]) -> Vec<u8> {
    let mut a = alloc::vec![0u8; (ENTRY_COUNT * ENTRY_SIZE) as usize];
    for (i, (region, id)) in Region::ALL.iter().zip(unique).enumerate() {
        let at = i * ENTRY_SIZE as usize;
        let e = &mut a[at..at + ENTRY_SIZE as usize];
        let extent = layout.extent(*region);
        e[0..16].copy_from_slice(&region.type_guid().0);
        e[16..32].copy_from_slice(&id.0);
        e[32..40].copy_from_slice(&extent.first.to_le_bytes());
        e[40..48].copy_from_slice(&extent.last().to_le_bytes());
        e[48..56].copy_from_slice(&region.attributes().to_le_bytes());
        for (j, ch) in region.name().encode_utf16().enumerate().take(36) {
            e[56 + j * 2..58 + j * 2].copy_from_slice(&ch.to_le_bytes());
        }
    }
    a
}
