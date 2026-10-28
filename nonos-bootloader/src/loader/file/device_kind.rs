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

//! What kind of device the loader was read from, told by the type and
//! subtype of each node of its volume's device path (UEFI 2.10, 10.3.4).
//! Only those two bytes are read, so the host proofs mount this file as it
//! is.

/// The kind of disk this boot started from, as the boot menu names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootDevice {
    /// A USB node anywhere on the path: a stick, or a disk in a USB case.
    UsbStick,
    /// An NVMe namespace or a SATA port, and no USB node.
    InternalDisk,
    /// Neither, or a path that could not be read: virtio, SCSI, an SD card.
    ThisDisk,
}

const MESSAGING: u8 = 0x03;
/* USB, USB class and USB WWID. */
const USB: [u8; 3] = [0x05, 0x0F, 0x10];
/* SATA and the NVMe namespace. */
const INTERNAL: [u8; 2] = [0x12, 0x17];

/// The kind the nodes, each as (type, subtype), describe. A USB node wins
/// over any other: a disk reached through USB is one a person plugged in.
pub fn classify(nodes: impl IntoIterator<Item = (u8, u8)>) -> BootDevice {
    let mut kind = BootDevice::ThisDisk;
    for (ty, sub) in nodes {
        if ty != MESSAGING {
            continue;
        }
        if USB.contains(&sub) {
            return BootDevice::UsbStick;
        }
        if INTERNAL.contains(&sub) {
            kind = BootDevice::InternalDisk;
        }
    }
    kind
}

impl BootDevice {
    /// The value the boot menu's platform panel shows beside STARTING FROM.
    pub const fn label(self) -> &'static [u8] {
        match self {
            BootDevice::UsbStick => b"USB STICK",
            BootDevice::InternalDisk => b"INTERNAL DISK",
            BootDevice::ThisDisk => b"THIS DISK",
        }
    }
}
