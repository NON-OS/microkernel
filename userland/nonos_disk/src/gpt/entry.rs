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

//! The entry array: 128 slots of 128 bytes, the first holding the ESP and
//! the rest zero. The name is what partition tools and firmware menus show,
//! and it is fixed, so a disk NONOS wrote can be told from one something
//! else wrote by reading sector 2.

use alloc::vec::Vec;

use super::layout::{Layout, ENTRY_COUNT, ENTRY_SIZE};
use crate::guid::Guid;

pub const PARTITION_NAME: &str = "NONOS-ESP";

pub fn build_array(layout: &Layout, partition_guid: Guid) -> Vec<u8> {
    let mut a = alloc::vec![0u8; (ENTRY_COUNT * ENTRY_SIZE) as usize];
    let e = &mut a[..ENTRY_SIZE as usize];
    e[0..16].copy_from_slice(&Guid::ESP.0);
    e[16..32].copy_from_slice(&partition_guid.0);
    e[32..40].copy_from_slice(&layout.esp_first_lba.to_le_bytes());
    e[40..48].copy_from_slice(&layout.esp_last_lba.to_le_bytes());
    // Attributes stay zero: not marked required, not legacy bootable.
    for (i, ch) in PARTITION_NAME.encode_utf16().enumerate().take(36) {
        e[56 + i * 2..58 + i * 2].copy_from_slice(&ch.to_le_bytes());
    }
    a
}
