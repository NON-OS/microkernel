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

//! A part a dead predecessor left running: both engines on, both rings
//! naming its frames, and Bus Master on since boot. The bring-up stops and
//! resets it, and sees both engines off, before the first Command write
//! that turns Bus Master on.

use nonos_libc::{entropy, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use super::live::live_part;
use super::memory::Memory;
use super::model::window;
use super::order::bus_master_after_reset_and_rings;
use super::probe::probe;
use crate::constants::regs::{REG_RCTL, REG_RDBAL, REG_TCTL, REG_TDBAL};
use crate::constants::status::{RCTL_EN, TCTL_EN};
use crate::init::finish;
use crate::setup::command;

#[test]
fn a_part_left_running_is_stopped_and_reset_before_bus_master_goes_on() {
    let _turn = entropy(true);
    let bar = window();
    let part = live_part(&bar);
    for (reg, value) in [(REG_RCTL, RCTL_EN), (REG_TCTL, TCTL_EN)] {
        bar.present32(reg, value);
    }
    bar.present32(REG_RDBAL, 0x7FF0_0000);
    bar.present32(REG_TDBAL, 0x7FF1_0000);
    set_command(MEM | BM);
    watch(probe(&bar, &part));
    command(9, 1, false).expect("the claim-time write is taken");
    let left = probe(&bar, &part)();
    assert!(left.rx_on && left.tx_on && !left.rings_own, "{left:?}");
    finish(Memory::new().driver(&bar)).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), part.resets());
}
