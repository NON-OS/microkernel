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

use super::backend::Backend;
use super::rows::{AHCI, NVME, USB_MSC, VIRTIO_BLK};

/// Asked in this order. A USB stick that carries NONOS is first: it is
/// there because someone brought it to boot a live session from it, and
/// that session keeps its state on the stick, not on an internal disk that
/// may hold an installed NONOS. The kernel is not told which disk the
/// firmware booted, so this order stands in for that answer. Then NVMe
/// before SATA, so a machine with an NVMe data disk and a SATA boot disk
/// settles on the NVMe one without reading the other. The block-wire
/// drivers (eMMC and SD) follow SATA, where eMMC was asked while the AHCI
/// capsule served it; virtio-blk last.
/// The cost: on a machine with an xHCI controller, every disk waits while
/// driver.usb_msc0 looks for a device, about 1.5 s when none is plugged in.
pub(super) fn order() -> impl Iterator<Item = Backend> {
    let row = |disk| Backend::of(disk, 0);
    [&USB_MSC, &NVME, &AHCI]
        .map(row)
        .into_iter()
        .chain(super::wire::rows())
        .chain([row(&VIRTIO_BLK)])
}
