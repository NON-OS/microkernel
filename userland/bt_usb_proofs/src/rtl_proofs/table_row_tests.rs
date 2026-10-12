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

//! Every row of the capsule's table as linux-firmware's files and Linux's
//! driver give it: the name, the ROM's answer, the project, the layout and
//! the file. A row typed wrong would leave its chip unpatched, or send it
//! another chip's file.

use super::rtl::chip::Format::{self, Realtech, RtbtCore};
use super::rtl::chips::CHIPS;

type Row = (&'static str, (u16, u16, u8), u8, Format, &'static str);

const ROWS: [Row; 13] = [
    ("RTL8723B", (0x8723, 0x000b, 0x06), 1, Realtech, "rtl8723b_fw.bin"),
    ("RTL8723D", (0x8723, 0x000d, 0x08), 9, Realtech, "rtl8723d_fw.bin"),
    ("RTL8821A", (0x8821, 0x000a, 0x06), 2, Realtech, "rtl8821a_fw.bin"),
    ("RTL8821C", (0x8821, 0x000c, 0x08), 10, Realtech, "rtl8821c_fw.bin"),
    ("RTL8761A", (0x8761, 0x000a, 0x06), 3, Realtech, "rtl8761a_fw.bin"),
    ("RTL8761BU", (0x8761, 0x000b, 0x0a), 14, Realtech, "rtl8761bu_fw.bin"),
    ("RTL8822B", (0x8822, 0x000b, 0x07), 8, Realtech, "rtl8822b_fw.bin"),
    ("RTL8822CU", (0x8822, 0x000c, 0x0a), 13, RtbtCore, "rtl8822cu_fw.bin"),
    ("RTL8852AU", (0x8852, 0x000a, 0x0b), 18, RtbtCore, "rtl8852au_fw.bin"),
    ("RTL8852BU", (0x8852, 0x000b, 0x0b), 20, RtbtCore, "rtl8852bu_fw.bin"),
    ("RTL8852CU", (0x8852, 0x000c, 0x0c), 25, RtbtCore, "rtl8852cu_fw_v2.bin"),
    ("RTL8851BU", (0x8851, 0x000b, 0x0c), 36, RtbtCore, "rtl8851bu_fw.bin"),
    ("RTL8852BTU", (0x8852, 0x0087, 0x0c), 47, RtbtCore, "rtl8852btu_fw.bin"),
];

#[test]
fn every_row_is_the_chip_s_own() {
    assert_eq!(CHIPS.len(), ROWS.len());
    for (c, (name, rom, project, format, file)) in CHIPS.iter().zip(ROWS) {
        assert_eq!(c.name, name);
        assert_eq!((c.lmp_subversion, c.hci_revision, c.hci_version), rom, "{name}");
        assert_eq!((c.project, c.format, c.firmware.file), (project, format, file), "{name}");
    }
}

#[test]
fn second_layout_chips_have_no_config() {
    for c in CHIPS.iter().filter(|c| c.format == RtbtCore) {
        assert!(c.config.is_none() && !c.config_needed, "{}", c.name);
    }
}
