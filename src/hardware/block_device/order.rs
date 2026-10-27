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

/// Asked in this order. The order does not choose the disk: the one whose
/// table holds the partition the loader names is kept wherever it comes
/// (select.rs). It decides three things. With no partition named, an MBR
/// stick or no record, the first NONOS disk is kept, so a USB stick that
/// carries NONOS comes before an internal disk that may hold an installed
/// one. A row refused before any other NONOS disk is seen ends the pass, so
/// while driver.usb_msc0 is still looking every pass stops at the stick. And
/// the first NONOS disk that does not hold the loader's partition is the one
/// settle.rs may keep, never when the loader read the store from its own
/// disk. Then NVMe before SATA, so a machine with an NVMe data disk and a
/// SATA boot disk settles on the NVMe one without reading the other. The
/// block-wire drivers (eMMC and SD) follow SATA, where eMMC was asked while
/// the AHCI capsule served it; virtio-blk last.
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
