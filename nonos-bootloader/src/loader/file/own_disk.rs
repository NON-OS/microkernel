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

/*
 * Which disks hold this loader's volume: one whose device path starts the
 * volume's, or whose GPT lists the partition the volume's Hard Drive node
 * names. The path alone can miss a stick whose disk and partition paths
 * differ, and the stick's store was then marked as another disk's.
 */

use alloc::vec::Vec;

use uefi::prelude::*;
use uefi::proto::device_path::DevicePath;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::Handle;

use super::boot_partition::boot_partition;
use super::gpt_names::names;
use super::own_volume::own_volume;

/// For each of `handles`, whether it is the whole disk the loader's volume
/// lies on. The volume's path and partition are read before any disk is
/// opened: the firmware closes every GetProtocol open of a handle by this
/// agent at once, so two scopes on one handle (the volume is also a BlockIO
/// handle) left the second close NOT_FOUND, which the uefi crate asserts on.
pub(super) fn ours(bs: &BootServices, handles: &[Handle]) -> Vec<bool> {
    let own = own_volume(bs);
    let own_nodes = own.and_then(|h| nodes(bs, h)).unwrap_or_default();
    let guid = boot_partition(bs).filter(|m| m.signature_type == 2).map(|m| m.signature);
    let named = |h: Handle| guid.is_some_and(|g| names(bs, h, &g));
    handles.iter().map(|&h| Some(h) != own && (holds(bs, h, &own_nodes) || named(h))).collect()
}

/* Each node of the device path of `handle`, as its bytes. */
fn nodes(bs: &BootServices, handle: Handle) -> Option<Vec<Vec<u8>>> {
    let params = OpenProtocolParams { handle, agent: bs.image_handle(), controller: None };
    /* SAFETY: GetProtocol borrows the interface for the life of `path`
     * alone, and nothing here uninstalls it while it is held. */
    let path = unsafe {
        bs.open_protocol::<DevicePath>(params, OpenProtocolAttributes::GetProtocol).ok()?
    };
    let out = path
        .node_iter()
        .map(|node| {
            // SAFETY: a node is `length()` bytes from its header on, as the firmware
            // laid the path out; they are copied while the protocol is open.
            unsafe {
                core::slice::from_raw_parts(node.as_ffi_ptr() as *const u8, node.length() as usize)
            }
            .to_vec()
        })
        .collect();
    Some(out)
}

/* Whether disk `disk` is the one the volume whose path is `own` lies on: its
 * device path is the start of the volume's. */
fn holds(bs: &BootServices, disk: Handle, own: &[Vec<u8>]) -> bool {
    let Some(disk) = nodes(bs, disk) else { return false };
    !disk.is_empty() && disk.len() < own.len() && disk.iter().zip(own).all(|(a, b)| a == b)
}
