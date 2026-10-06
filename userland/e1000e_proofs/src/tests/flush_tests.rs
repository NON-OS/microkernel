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

//! e1000_flush_desc_rings before the reset on pch_spt and later: done only
//! when config word 0xE4 asks and a ring is programmed, the transmit flush
//! through this driver's own ring, the receive flush only if still asked.

use nonos_libc::{config_reads, set_ring_status};

use crate::constants::pch_bits::FEXTNVM11_DISABLE_MULR_FIX;
use crate::constants::queue::TX_CMD_IFCS;
use crate::constants::regs::*;
use crate::constants::regs_pch::REG_FEXTNVM11;
use crate::constants::rxtx::{RCTL_EN, RXDCTL_THRESH_UNIT_DESC, TCTL_EN};
use crate::constants::Family;
use crate::init::flush_rings;
use crate::model::memory::{Memory, TX_RING_PHYS};
use crate::model::window::pch;

#[test]
fn a_ring_firmware_left_pending_is_flushed_through_the_drivers_own_ring() {
    let bar = pch();
    bar.present32(REG_TDLEN, 4096);
    bar.present32(REG_RCTL, RCTL_EN);
    set_ring_status(0x100);
    let mut mem = Memory::new();
    flush_rings::run(&mem.driver(&bar, Family::PchCnp, 0x15BB));
    assert_ne!(bar.wrote32(REG_FEXTNVM11) & FEXTNVM11_DISABLE_MULR_FIX, 0);
    assert_eq!((bar.wrote32(REG_TDBAL), bar.wrote32(REG_TDLEN)), (TX_RING_PHYS as u32, 512));
    assert_eq!((bar.wrote32(REG_TDH), bar.wrote32(REG_TDT)), (0, 1));
    assert_ne!(bar.wrote32(REG_TCTL) & TCTL_EN, 0);
    let d = mem.tx[0];
    assert_eq!((d.buffer_addr, d.length, d.cmd), (TX_RING_PHYS, 512, TX_CMD_IFCS));
    // Still asked after the transmit flush: the receive flush runs too.
    assert_eq!(bar.wrote32(REG_RXDCTL) & 0x3FFF, 0x11F);
    assert_ne!(bar.wrote32(REG_RXDCTL) & RXDCTL_THRESH_UNIT_DESC, 0);
    assert_eq!(bar.wrote32(REG_RCTL) & RCTL_EN, 0, "the receiver is left off");
    assert_eq!(config_reads(), [(0xE4, 2), (0xE4, 2)]);
}

#[test]
fn nothing_is_flushed_unless_the_part_asks_and_a_ring_exists() {
    for (status, tdlen) in [(0, 4096), (0x100, 0)] {
        let bar = pch();
        bar.present32(REG_TDLEN, tdlen);
        set_ring_status(status);
        let mut mem = Memory::new();
        flush_rings::run(&mem.driver(&bar, Family::PchMtp, 0x550A));
        assert_eq!(bar.wrote32(REG_TDT), 0);
        assert_eq!(mem.tx[0].length, 0);
    }
}
