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

//! How much memory the machine has, as the firmware's map counts it.

use uefi::prelude::*;
use uefi::table::boot::MemoryType;

use super::super::pages::{Pages, PAGE};

/// The bytes of every range the kernel may come to use: free memory and what
/// the firmware and this loader hold until the kernel runs. 0 when the map
/// could not be read, which copies nothing.
pub(super) fn machine_bytes(bs: &BootServices) -> u64 {
    let size = bs.memory_map_size();
    let len = size.map_size + size.entry_size * 8;
    let Some(buf) = Pages::new(bs, len.div_ceil(PAGE)) else { return 0 };
    let Ok(map) = bs.memory_map(&mut buf.bytes()[..len]) else { return 0 };
    map.entries()
        .filter(|d| {
            matches!(
                d.ty,
                MemoryType::CONVENTIONAL
                    | MemoryType::LOADER_CODE
                    | MemoryType::LOADER_DATA
                    | MemoryType::BOOT_SERVICES_CODE
                    | MemoryType::BOOT_SERVICES_DATA
            )
        })
        .map(|d| d.page_count * PAGE as u64)
        .sum()
}
