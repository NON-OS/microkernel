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

//! MAC OCP encoding and the 8168g init (Linux __r8168_mac_ocp_write and
//! rtl_hw_init_8168g). MAC OCP has no handshake, so a passive window shows
//! exactly what was written; a read of it returns 0 in the low half.

use nonos_libc::logged;

use super::model::window;
use crate::chip::MacVersion;
use crate::constants::regs::{REG_CMD, REG_TX_CONFIG};
use crate::hw::init_8168g;
use crate::hw::ocp::{mac_ocp_read, mac_ocp_write};
use crate::hw::regs::{MISC_RXDV_GATED_EN, REG_MCU, REG_MISC, TXCFG_EMPTY};
use crate::regs::Regs;

const OCPDR: usize = 0xB0;

#[test]
fn a_mac_ocp_write_is_flag_register_shifted_by_15_and_data() {
    let bar = window();
    let regs = Regs::new(bar.base());
    mac_ocp_write(&regs, 0xE8DE, 0x1234);
    assert_eq!(bar.wrote32(OCPDR), 0x8000_0000 | (0xE8DE << 15) | 0x1234);
    assert_eq!(mac_ocp_read(&regs, 0xC0AA), 0);
    assert_eq!(bar.wrote32(OCPDR), 0xC0AA << 15, "a read writes the register alone");
    mac_ocp_write(&regs, 0xE8DF, 1);
    assert_eq!(bar.wrote32(OCPDR), 0xC0AA << 15, "an odd register is not written");
    assert!(logged().iter().any(|l| l.contains("MAC OCP register 0xe8df is not valid")));
}

#[test]
fn the_8168g_init_gates_rx_stops_both_engines_leaves_oob_and_rebuilds_the_link_list() {
    let bar = window();
    bar.present8(REG_CMD, 0x0C);
    bar.present8(REG_MCU, 0x80 | 0x30 | 0x02);
    bar.present32(REG_TX_CONFIG, TXCFG_EMPTY);
    init_8168g(&Regs::new(bar.base()), MacVersion(46));
    assert_eq!(bar.wrote8(REG_CMD) & 0x0C, 0, "TE and RE off");
    assert_eq!(bar.wrote8(REG_MCU) & 0x80, 0, "NOW_IS_OOB cleared");
    assert_ne!(bar.wrote32(REG_MISC) & MISC_RXDV_GATED_EN, 0, "RXDV gate on");
    let last = 0x8000_0000 | (0xE8DE << 15) | (1 << 15);
    assert_eq!(bar.wrote32(OCPDR), last, "0xE8DE bit 15 set last");
    assert!(!logged().iter().any(|l| l.contains("LINK_LIST_RDY")), "{:?}", logged());
}
