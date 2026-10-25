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

//! The scope rows (scope.rs): each driver the kernel spawns, by the service name its
//! spawn.rs registers, and what it may claim. The USB function drivers, i2c_hid and
//! spi_hid claim nothing: they reach their devices through a host's driver, so they
//! have no row; nor has vmmouse0, which speaks only the backdoor. The ACPI capsule
//! claims the SCI record alone (acpi/sci.rs).

use super::hosts::{SDHCI, SPI, XHCI};
use super::ids_nic::{E1000, E1000E, IGB, IGC, IWLWIFI, IXGBE, RTL8169};
use super::rows::{
    AHCI, ANY, HDA, I2C_PCI, INTEL, INTEL_GFX, PS2, REALTEK, SVGA, VBOX, VIRTIO, VMWARE,
};
use super::rule::Match::{self, Acpi, Class, Ids, Virt};

pub(super) const SCOPE: &[(&str, &[Match])] = &[
    ("driver.e1000_0", &[Ids(INTEL, E1000)]),
    ("driver.e1000e_0", &[Ids(INTEL, E1000E)]),
    ("driver.igb_0", &[Ids(INTEL, IGB)]),
    ("driver.igc_0", &[Ids(INTEL, IGC)]),
    ("driver.ixgbe_0", &[Ids(INTEL, IXGBE)]),
    ("driver.rtl8169_0", &[Ids(REALTEK, RTL8169)]),
    ("driver.rtl8139_0", &[Ids(REALTEK, &[0x8139, 0x8139])]),
    ("driver.virtio_net0", &[Ids(VIRTIO, &[0x1000, 0x1000, 0x1041, 0x1041])]),
    ("driver.iwlwifi0", &[Ids(INTEL, IWLWIFI)]),
    ("driver.rtl8821ce0", &[Ids(REALTEK, &[0xC821, 0xC821])]),
    // Storage, SD and USB hosts bind by class, as their capsules do (ufs0's takes listed hosts).
    ("driver.nvme0", &[Class { vendor: ANY, class: 0x01, sub: 0x08, progif: Some(0x02) }]),
    ("driver.ahci0", AHCI),
    ("driver.sdhci0", SDHCI),
    ("driver.ufs0", &[Class { vendor: ANY, class: 0x01, sub: 0x09, progif: None }]),
    ("driver.virtio_blk0", &[Ids(VIRTIO, &[0x1001, 0x1001, 0x1042, 0x1042])]),
    ("driver.xhci0", XHCI),
    ("driver.ehci0", &[Class { vendor: ANY, class: 0x0C, sub: 0x03, progif: Some(0x20) }]),
    ("driver.ohci0", &[Class { vendor: ANY, class: 0x0C, sub: 0x03, progif: Some(0x10) }]),
    ("driver.uhci0", &[Class { vendor: ANY, class: 0x0C, sub: 0x03, progif: Some(0x00) }]),
    ("driver.smbus0", &[Class { vendor: Some(INTEL), class: 0x0C, sub: 0x05, progif: None }]),
    ("driver.spi0", SPI),
    ("driver.i2c_pci0", I2C_PCI),
    ("driver.ps2_kbd0", PS2),
    ("driver.virtio_rng", &[Ids(VIRTIO, &[0x1005, 0x1005, 0x1044, 0x1044])]),
    ("driver.virtio_gpu0", &[Ids(VIRTIO, &[0x1050, 0x1050])]),
    ("driver.bga0", &[Ids(0x1234, &[0x1111, 0x1111])]),
    ("driver.intel_display0", INTEL_GFX),
    ("driver.hda0", HDA),
    ("driver.ac97_0", &[Ids(INTEL, &[0x2415, 0x2415])]),
    ("driver.buslogic0", &[Ids(0x104B, &[0x1040, 0x1040])]),
    ("driver.gpio0", &[Acpi { class: 0x0080, vendor: ANY, device: ANY }]),
    ("driver.mpt0", &[Ids(0x1000, &[0x0030, 0x0030, 0x0054, 0x0054])]),
    ("driver.pcnet0", &[Ids(0x1022, &[0x2000, 0x2000])]),
    ("driver.piix_ide0", &[Ids(INTEL, &[0x7010, 0x7010, 0x7111, 0x7111])]),
    ("driver.pvscsi0", &[Ids(VMWARE, &[0x07C0, 0x07C0])]),
    ("driver.storvsc0", &[Virt { vendor: 0x1414, device: 0x5343 }]),
    ("driver.tulip0", &[Ids(0x1011, &[0x0009, 0x0009, 0x0019, 0x0019])]),
    ("driver.virtio_input0", &[Ids(VIRTIO, &[0x1052, 0x1052])]),
    ("driver.virtio_scsi0", &[Ids(VIRTIO, &[0x1004, 0x1004, 0x1048, 0x1048])]),
    ("driver.vmbus0", &[Virt { vendor: 0x1414, device: 0x5642 }]),
    ("driver.vmmdev0", &[Ids(VBOX, &[0xCAFE, 0xCAFE])]),
    ("driver.vmware_svga0", SVGA),
    ("driver.vmxnet3_0", &[Ids(VMWARE, &[0x07B0, 0x07B0])]),
    ("acpi", &[Acpi { class: 0xFFFF, vendor: Some(0x0001), device: Some(0x0C08) }]),
];
