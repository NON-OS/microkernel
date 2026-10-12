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

//! The scope rows (scope.rs): each driver the kernel spawns, by the service
//! name its spawn.rs registers, and what it may claim. usb_hid, usb_msc and
//! i2c_hid claim nothing: they reach their devices through xhci0 and
//! i2c_pci0, so they have no row.

use super::ids::{E1000, IWLWIFI, RTL8169};
use super::rows::{AHCI, ANY, HDA, I2C_PCI, INTEL, PS2};
use super::rule::Match::{self, Class, Ids};

const REALTEK: u16 = 0x10EC;
const VIRTIO: u16 = 0x1AF4;

pub(super) const SCOPE: &[(&str, &[Match])] = &[
    ("driver.e1000_0", &[Ids(INTEL, E1000)]),
    ("driver.rtl8169_0", &[Ids(REALTEK, RTL8169)]),
    ("driver.rtl8139_0", &[Ids(REALTEK, &[0x8139, 0x8139])]),
    ("driver.virtio_net0", &[Ids(VIRTIO, &[0x1000, 0x1000, 0x1041, 0x1041])]),
    ("driver.iwlwifi0", &[Ids(INTEL, IWLWIFI)]),
    ("driver.rtl8821ce0", &[Ids(REALTEK, &[0xC821, 0xC821])]),
    // NVMe, AHCI and HDA bind by class, from any vendor, as their capsules do.
    ("driver.nvme0", &[Class { vendor: ANY, class: 0x01, sub: 0x08, progif: Some(0x02) }]),
    ("driver.ahci0", AHCI),
    ("driver.virtio_blk0", &[Ids(VIRTIO, &[0x1001, 0x1001, 0x1042, 0x1042])]),
    ("driver.xhci0", &[Class { vendor: ANY, class: 0x0C, sub: 0x03, progif: Some(0x30) }]),
    ("driver.i2c_pci0", I2C_PCI),
    ("driver.ps2_kbd0", PS2),
    ("driver.virtio_rng", &[Ids(VIRTIO, &[0x1005, 0x1005, 0x1044, 0x1044])]),
    ("driver.virtio_gpu0", &[Ids(VIRTIO, &[0x1010, 0x1010, 0x1050, 0x1050])]),
    ("driver.hda0", HDA),
];
