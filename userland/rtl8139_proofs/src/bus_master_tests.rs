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

//! Bus mastering against a part that has been reset: on only once the part
//! is stopped and the rings name this driver's frames, and off, with the
//! part stopped and reset, before anything goes back to the broker.

use nonos_libc::{entropy, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use crate::constants::regs::REG_RCR;
use crate::init::finish;
use crate::order::{bus_master_after_reset_and_rings, released_quiet};
use crate::part::{driver_over, part_with, DEVICE};
use crate::probe::probe;
use crate::setup::command;

/// As the kernel's boot pass leaves every endpoint, and the claim with it.
const BOOT: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;

/// The claim, and the setup sequence's first write after it.
fn claim(resets: &std::rc::Rc<std::cell::Cell<u32>>) {
    set_command(BOOT);
    watch(probe(resets));
    command(DEVICE, 1, false).expect("the claim-time write is taken");
}

#[test]
fn bus_mastering_goes_on_only_after_the_reset_and_the_rings() {
    let _turn = entropy(true);
    let (_port, resets) = part_with(false, None);
    claim(&resets);
    finish(driver_over(0)).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets.get());
}

#[test]
fn a_driver_torn_down_stops_and_resets_the_part_and_clears_bus_master_first() {
    let _turn = entropy(true);
    let (_port, resets) = part_with(false, None);
    claim(&resets);
    let driver = finish(driver_over(0)).expect("brought up");
    let (mark, done) = (seen().len(), resets.get());
    driver.release();
    released_quiet(&seen()[mark..], done + 1);
}

/// The narrow case: a port write refused after both engines are on.
#[test]
fn a_write_refused_with_the_receiver_on_stops_the_part_before_release() {
    let _turn = entropy(true);
    let (_port, resets) = part_with(false, Some(REG_RCR));
    claim(&resets);
    assert_eq!(finish(driver_over(0)).err(), Some("pio write failed"));
    released_quiet(&seen(), 2);
}
