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

use nonos_libc::{entropy, seen, set_command, watch, MK_PCI_CMD_MEMORY_SPACE};

use super::live::live_part;
use super::memory::Memory;
use super::model::window;
use super::order::{bus_master_after_reset_and_rings, released_quiet};
use super::probe::probe;
use crate::init::finish;

/// The second attempt after one that failed and gave the device back, which
/// cleared Bus Master: the bring-up has to turn it on itself.
#[test]
fn a_second_attempt_turns_bus_mastering_on_after_the_reset_and_the_rings() {
    let _turn = entropy(true);
    let bar = window();
    set_command(MK_PCI_CMD_MEMORY_SPACE);
    let mut mem = Memory::new();
    assert_eq!(finish(mem.driver(&bar)).err(), Some("CTRL.RST did not self-clear"));
    let part = live_part(&bar);
    watch(probe(&bar, &part));
    finish(mem.driver(&bar)).expect("the second attempt comes up");
    bus_master_after_reset_and_rings(&seen(), part.resets());
}

#[test]
fn a_driver_torn_down_stops_and_resets_the_part_and_clears_bus_master_first() {
    let _turn = entropy(true);
    let bar = window();
    let part = live_part(&bar);
    set_command(MK_PCI_CMD_MEMORY_SPACE);
    watch(probe(&bar, &part));
    let mut mem = Memory::new();
    let driver = finish(mem.driver(&bar)).expect("brought up");
    let (mark, resets) = (seen().len(), part.resets());
    driver.release();
    released_quiet(&seen()[mark..], resets + 1);
}
