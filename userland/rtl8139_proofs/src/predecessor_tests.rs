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

//! A part a dead predecessor left running: both engines on, the receive
//! buffer and the first transmit slot naming its frames, and Bus Master on
//! since boot. The bring-up resets it, and sees both engines off, before
//! the first Command write that turns Bus Master on.

use std::{cell::Cell, rc::Rc};

use nonos_libc::{attach, entropy, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use crate::constants::regs::{CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD, REG_RBSTART, REG_TXADDR0};
use crate::init::finish;
use crate::order::bus_master_after_reset_and_rings;
use crate::part::{driver_over, Model, DEVICE};
use crate::probe::probe;
use crate::setup::command;

#[test]
fn a_part_left_running_is_reset_before_bus_master_goes_on() {
    let _turn = entropy(true);
    let mut regs = [0u8; 256];
    regs[REG_CMD as usize] = CMD_RX_ENABLE | CMD_TX_ENABLE;
    regs[REG_RBSTART as usize + 3] = 0x7F;
    regs[REG_TXADDR0 as usize + 3] = 0x7E;
    let resets = Rc::new(Cell::new(0));
    let model = Model { regs, stuck_in_reset: false, refuse: None, resets: Rc::clone(&resets) };
    let _port = attach(Box::new(model));
    set_command(MEM | BM);
    watch(probe(&resets));
    command(DEVICE, 1, false).expect("the claim-time write is taken");
    let left = probe(&resets)();
    assert!(left.rx_on && left.tx_on && !left.rings_own, "{left:?}");
    finish(driver_over(0)).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets.get());
}
