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

//! The Realtek USB controllers whose patch file is the first layout
//! ("Realtech"): what each ROM answers, its project, and its pinned files.
//! The second layout ("RTBTCore": RTL8822CU, RTL8852A, B, BT and C,
//! RTL8851B) has sections and keys of its own and is a later unit.

use super::chip::Chip;
use super::pin::Pin;
use super::pins::*;
use super::pins_config::*;

pub const CHIPS: &[Chip] = &[
    usb("RTL8723B", (0x8723, 0x000b, 0x06), 1, &RTL8723B_FW, &RTL8723BS_CONFIG, false),
    usb("RTL8723D", (0x8723, 0x000d, 0x08), 9, &RTL8723D_FW, &RTL8821C_CONFIG, true),
    usb("RTL8821A", (0x8821, 0x000a, 0x06), 2, &RTL8821A_FW, &RTL8821C_CONFIG, false),
    usb("RTL8821C", (0x8821, 0x000c, 0x08), 10, &RTL8821C_FW, &RTL8821C_CONFIG, false),
    usb("RTL8761A", (0x8761, 0x000a, 0x06), 3, &RTL8761A_FW, &RTL8761BU_CONFIG, false),
    usb("RTL8761BU", (0x8761, 0x000b, 0x0a), 14, &RTL8761BU_FW, &RTL8761BU_CONFIG, false),
    usb("RTL8822B", (0x8822, 0x000b, 0x07), 8, &RTL8822B_FW, &RTL8822B_CONFIG, true),
];

/// One row: LMP subversion, HCI revision and HCI version as the ROM
/// answers them, then the project, the patch, the config and whether the
/// config is needed.
const fn usb(
    name: &'static str,
    (lmp_subversion, hci_revision, hci_version): (u16, u16, u8),
    project: u8,
    firmware: &'static Pin,
    config: &'static Pin,
    config_needed: bool,
) -> Chip {
    Chip {
        name,
        lmp_subversion,
        hci_revision,
        hci_version,
        project,
        firmware,
        config: Some(config),
        config_needed,
    }
}
