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
 * The kind of disk this loader was read from, so the boot menu can say
 * whether a USB stick or an internal disk started: after an install the two
 * show the same menu. The volume's device path is opened with GetProtocol,
 * as boot_partition.rs opens it, and closed before this returns, so no
 * other open of the same handle is outstanding when the scope ends.
 */

use uefi::prelude::*;
use uefi::proto::device_path::DevicePath;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};

use super::device_kind::{classify, BootDevice};
use super::own_volume::own_volume;

pub fn boot_device(bs: &BootServices) -> BootDevice {
    let Some(handle) = own_volume(bs) else { return BootDevice::ThisDisk };
    let params = OpenProtocolParams { handle, agent: bs.image_handle(), controller: None };
    /* SAFETY: GetProtocol borrows the interface for the life of `path`
     * alone, and nothing here uninstalls it while it is held. */
    let path =
        unsafe { bs.open_protocol::<DevicePath>(params, OpenProtocolAttributes::GetProtocol) };
    let Ok(path) = path else { return BootDevice::ThisDisk };
    classify(path.node_iter().map(|node| (node.device_type().0, node.sub_type().0)))
}
