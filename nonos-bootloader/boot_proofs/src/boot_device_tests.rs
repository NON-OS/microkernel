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

//! The boot menu says whether a USB stick or an internal disk started, from
//! the loader's device path: after an install the two show the same menu.
//! The paths are the ones firmware builds (UEFI 2.10, 10.3), as (type,
//! subtype) per node, ending with the Hard Drive node and the end node.

use crate::loader::device_kind::{classify, BootDevice};

const PCI_ROOT: (u8, u8) = (0x02, 0x01);
const PCI: (u8, u8) = (0x01, 0x01);
const HARD_DRIVE: (u8, u8) = (0x04, 0x01);
const END: (u8, u8) = (0x7F, 0xFF);

fn path(middle: &[(u8, u8)]) -> Vec<(u8, u8)> {
    [&[PCI_ROOT, PCI][..], middle, &[HARD_DRIVE, END]].concat()
}

#[test]
fn a_usb_node_is_a_stick_however_it_is_named() {
    for sub in [0x05, 0x0F, 0x10] {
        assert_eq!(classify(path(&[(0x03, sub)])), BootDevice::UsbStick, "subtype {sub:#x}");
    }
    // Behind a hub, and an NVMe bridge in a USB case: still plugged in.
    assert_eq!(classify(path(&[(0x03, 0x05), (0x03, 0x05)])), BootDevice::UsbStick);
    assert_eq!(classify(path(&[(0x03, 0x05), (0x03, 0x17)])), BootDevice::UsbStick);
    assert_eq!(classify(path(&[(0x03, 0x17), (0x03, 0x05)])), BootDevice::UsbStick);
}

#[test]
fn nvme_and_sata_are_internal_disks() {
    assert_eq!(classify(path(&[(0x03, 0x17)])), BootDevice::InternalDisk, "NVMe namespace");
    assert_eq!(classify(path(&[(0x03, 0x12)])), BootDevice::InternalDisk, "SATA port");
}

#[test]
fn anything_else_is_this_disk() {
    assert_eq!(classify(path(&[])), BootDevice::ThisDisk, "virtio-blk: PCI alone");
    assert_eq!(classify(path(&[(0x03, 0x02)])), BootDevice::ThisDisk, "SCSI");
    assert_eq!(classify(path(&[(0x03, 0x1A)])), BootDevice::ThisDisk, "SD card");
    assert_eq!(classify(Vec::new()), BootDevice::ThisDisk, "no path read");
    // A USB or NVMe subtype under another type is not that device.
    assert_eq!(classify(path(&[(0x01, 0x05), (0x04, 0x17)])), BootDevice::ThisDisk);
}

#[test]
fn the_menu_names_each_kind() {
    assert_eq!(BootDevice::UsbStick.label(), b"USB STICK");
    assert_eq!(BootDevice::InternalDisk.label(), b"INTERNAL DISK");
    assert_eq!(BootDevice::ThisDisk.label(), b"THIS DISK");
}
