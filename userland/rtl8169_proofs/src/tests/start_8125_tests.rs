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

//! The 8125 start against Linux rtl_hw_start_8125 and
//! rtl_hw_start_8125_common, in a 64 KiB window as the 8125's BAR is.

use nonos_devmodel::FakeBar;
use nonos_libc::logged;

use crate::chip::MacVersion;
use crate::hw::ocp_8125::{by_version, FORMAT};
use crate::hw::regs::{MISC_RXDV_GATED_EN, REG_MISC};
use crate::hw::start_8125;
use crate::regs::Regs;

fn started(ver: u8) -> FakeBar {
    let bar = FakeBar::new(0x10000);
    bar.present8(0x34, 0xFF);
    bar.present32(0xA00, 0xFFFF_FFFF);
    bar.present32(0xA7C, 0xFFFF_FFFF);
    bar.present32(0xA80, 0xDEAD_BEEF);
    bar.present16(0x7A, 0xFFFF);
    bar.present8(0x54, 0xFF);
    bar.present8(0x52, 0xFF);
    bar.present16(0x1880, 0xFFFF);
    bar.present32(REG_MISC, MISC_RXDV_GATED_EN);
    start_8125(&Regs::new(bar.base()), MacVersion(ver));
    bar
}

#[test]
fn the_8125b_start_writes_linux_registers_and_releases_the_rx_gate() {
    let bar = started(63);
    assert_eq!(bar.wrote8(0x34), 0, "INT_CFG0_8125");
    assert_eq!((bar.wrote32(0xA00), bar.wrote32(0xA7C)), (0, 0), "coalescing 0xA00..0xA80");
    assert_eq!(bar.wrote32(0xA80), 0xDEAD_BEEF, "the 8125B stops at 0xA80");
    assert_eq!(bar.wrote16(0x7A), 0, "INT_CFG1_8125");
    assert_eq!(bar.wrote8(0x54), 0xFD, "Config3 Rdy_to_L23 cleared");
    assert_eq!(bar.wrote8(0x52), 0xEF, "Config1 bit 4 cleared");
    assert_eq!(bar.wrote16(0x382), 0x221B);
    assert_eq!((bar.wrote32(0x4500), bar.wrote16(0x4800)), (0, 0), "RSS and queue count off");
    assert_eq!(bar.wrote16(0x1880), 0xFFCF);
    assert_eq!(bar.wrote32(REG_MISC) & MISC_RXDV_GATED_EN, 0, "RXDV gate released");
    assert!(!logged().iter().any(|l| l.contains("0xE00E")), "{:?}", logged());
}

#[test]
fn the_8125a_clears_the_whole_coalescing_block_and_keeps_int_cfg1() {
    let bar = started(61);
    assert_eq!((bar.wrote32(0xA7C), bar.wrote32(0xA80)), (0, 0), "0xA00..0xB00");
    assert_eq!(bar.wrote16(0x7A), 0xFFFF, "INT_CFG1_8125 is the 8125B's");
}

#[test]
fn the_ocp_table_turns_the_new_descriptor_format_off_and_keys_e614_on_the_8125b() {
    assert!(FORMAT.contains(&(0xEB58, 0x0001, 0x0000)), "0xEB58 bit 0 cleared");
    assert_eq!(by_version(63), [(0xE614, 0x0700, 0x0200), (0xE63E, 0x0C30, 0x0000)]);
    for v in [61u8, 64, 65, 66] {
        assert_eq!(by_version(v), [(0xE614, 0x0700, 0x0300), (0xE63E, 0x0C30, 0x0020)]);
    }
}
