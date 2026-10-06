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
 * The partition this loader was read from, taken from the Hard Drive node
 * of its volume's device path. Opened with GetProtocol because the disk
 * driver already holds the path BY_DRIVER, and an exclusive open would
 * disconnect it. `None` when the volume has no partition node, as on a
 * medium formatted whole with no table.
 */

use uefi::prelude::*;
use uefi::proto::device_path::media::{HardDrive, PartitionSignature};
use uefi::proto::device_path::{DevicePath, DevicePathNodeEnum};
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};

use super::own_volume::own_volume;
use crate::handoff::types::BootMedia;

pub fn boot_partition(bs: &BootServices) -> Option<BootMedia> {
    let handle = own_volume(bs)?;
    let params = OpenProtocolParams { handle, agent: bs.image_handle(), controller: None };
    /* SAFETY: GetProtocol borrows the interface for the life of `path`
     * alone, and nothing here uninstalls it while it is held. */
    let path = unsafe {
        bs.open_protocol::<DevicePath>(params, OpenProtocolAttributes::GetProtocol).ok()?
    };
    path.node_iter().find_map(|node| match node.as_enum() {
        Ok(DevicePathNodeEnum::MediaHardDrive(hd)) => Some(record(hd)),
        _ => None,
    })
}

fn record(hd: &HardDrive) -> BootMedia {
    let mut signature = [0u8; 16];
    let signature_type = match hd.partition_signature() {
        PartitionSignature::Mbr(s) => {
            signature[..4].copy_from_slice(&s);
            1
        }
        PartitionSignature::Guid(g) => {
            signature = g.to_bytes();
            2
        }
        _ => 0,
    };
    BootMedia {
        partition_number: hd.partition_number(),
        table: hd.partition_format().0,
        signature_type,
        reserved: 0,
        start_lba: hd.partition_start(),
        size_lba: hd.partition_size(),
        signature,
    }
}
