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

//! The 57765-class functions, from tg3_pci_tbl and tg3.h's TG3PCI_DEVICE_*.
//! tg3_detect_asic_rev reads all ten parts' revision from the Gen 1.5
//! product-ID register, and each comes out ASIC_REV_57765 or _57766.

pub const BROADCOM_VENDOR_ID: u16 = 0x14E4;

/// (device, 10/100 only). tg3_pci_tbl gives the 57791 and 57795
/// TG3_DRV_DATA_FLAG_10_100_ONLY, and tg3_10_100_only_device takes that
/// for any part but the 5705.
pub const TG3_PARTS: &[(u16, bool)] = &[
    (0x16B1, false), // TG3PCI_DEVICE_TIGON3_57781
    (0x16B5, false), // TG3PCI_DEVICE_TIGON3_57785
    (0x16B0, false), // TG3PCI_DEVICE_TIGON3_57761
    (0x16B4, false), // TG3PCI_DEVICE_TIGON3_57765
    (0x16B2, true),  // TG3PCI_DEVICE_TIGON3_57791
    (0x16B6, true),  // TG3PCI_DEVICE_TIGON3_57795
    (0x1682, false), // TG3PCI_DEVICE_TIGON3_57762
    (0x1686, false), // TG3PCI_DEVICE_TIGON3_57766
    (0x16B7, false), // TG3PCI_DEVICE_TIGON3_57782
    (0x16B3, false), // TG3PCI_DEVICE_TIGON3_57786
];

/// Whether `device` is one of them, and if so whether it is 10/100 only.
pub fn part_of(device: u16) -> Option<bool> {
    TG3_PARTS.iter().find(|(id, _)| *id == device).map(|(_, fe)| *fe)
}

/// tg3_init_one maps pci_ioremap_bar(pdev, BAR_0): the register file.
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
