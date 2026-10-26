// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The device tree the firmware installed, from the UEFI configuration
//! table, for x0 at the kernel's entry.

use uefi::prelude::*;
use uefi::{guid, Guid};

use super::fdt_header::fdt_size;

/// The UEFI specification's configuration-table GUID for a flattened tree.
const DTB_GUID: Guid = guid!("b1b621d5-f19c-41a5-830b-d9152c69aae0");
#[derive(Clone, Copy)]
pub struct DeviceTree {
    pub addr: u64,
    pub len: u64,
}

/// The firmware's tree; None when the table has none, or the entry is null,
/// misaligned (the specification puts the tree on 8 bytes) or no tree.
pub fn device_tree(st: &SystemTable<Boot>) -> Option<DeviceTree> {
    let entry = st.config_table().iter().find(|e| e.guid == DTB_GUID)?;
    let addr = entry.address as u64;
    if addr == 0 || !addr.is_multiple_of(8) {
        return None;
    }
    // SAFETY: the firmware's table names memory it installed and keeps;
    // boot services run with memory mapped one to one, and the read is 8
    // aligned bytes at a non-null, 8 aligned address.
    let head = unsafe { core::ptr::read_unaligned(addr as *const [u8; 8]) };
    Some(DeviceTree { addr, len: fdt_size(head)? })
}
