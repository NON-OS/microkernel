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

//! tg3_chip_reset for the 57765 class on a window with no bootcode: the
//! core-clock reset with the PCI Express bit, configuration put back in
//! Linux's order, the bootcode waited for the whole bound, then the swap
//! modes and a quiet MAC; and Command put back in only the bits it may.

use nonos_libc::{config_set, config_writes, said};

use crate::constants::grc::*;
use crate::constants::mac::MAC_MODE;
use crate::constants::mi::{MAC_MI_MODE, MAC_MI_MODE_500KHZ_CONST};
use crate::constants::pcicfg::*;
use crate::constants::timeouts::POLL_FW_MS;
use crate::init::reset::{chip_reset, pcie};
use crate::setup::config::Config;

use super::build::{LOM, PCIE_CAP, REV_57766};
use super::config_replay::seed;
use super::memory::Memory;
use super::model::{timed, window};

#[test]
fn reset_puts_configuration_back_in_linuxs_order() {
    let bar = window();
    let mut m = Memory::new();
    let mut d = m.driver(&bar, REV_57766, LOM);
    seed();
    let (r, took) = timed(|| chip_reset(&mut d));
    assert_eq!(r, Ok(()));
    assert!(took.as_millis() as u64 >= POLL_FW_MS);
    let at: Vec<u32> = config_writes().iter().map(|(o, _)| *o).collect();
    let devctl = PCIE_CAP + PCI_EXP_DEVCTL;
    let devsta = PCIE_CAP + PCI_EXP_DEVSTA;
    assert_eq!(at, [devctl, devsta, TG3PCI_MISC_HOST_CTRL, TG3PCI_PCISTATE, 0x04]);
    let w = config_writes();
    assert_eq!((w[0].1, w[1].1, w[3].1, w[4].1), (0x2000, 0x000F, 0x0060, 0x0006));
    assert_eq!(bar.wrote32(GRC_MISC_CFG), GRC_MISC_CFG_CORECLK_RESET | GRC_MISC_CFG_PCIE_BIT29);
    assert_eq!(bar.wrote32(GRC_FASTBOOT_PC), 0);
    assert_eq!(bar.wrote32(GRC_MODE), GRC_MODE_BASE);
    assert_eq!(bar.wrote32(MAC_MODE), 0);
    assert_eq!(bar.wrote32(MAC_MI_MODE), MAC_MI_MODE_500KHZ_CONST);
    assert!(said().contains(&"tg3: no firmware running\n".to_string()));
}

#[test]
fn command_gets_back_only_the_bits_the_broker_lets_move() {
    let cfg = Config { device_id: 9, epoch: 1 };
    for (now, saved, want) in [(0x0000, 0xFFFF, 0x0406), (0x0100, 0x0002, 0x0102)] {
        config_set(0x04, now, 2);
        assert_eq!(pcie::restore(&cfg, saved), Ok(()));
        assert_eq!(config_writes().last(), Some(&(0x04, want)));
    }
}
