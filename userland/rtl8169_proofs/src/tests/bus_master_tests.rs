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

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{entropy, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::memory::Memory;
use super::model::{live, window};
use super::order::{bus_master_after_reset_and_rings, released_quiet};
use super::probe::{counting, probe};
use crate::init::finish;
use crate::setup::command;

/// As the kernel's boot pass leaves every endpoint, and the claim with it.
const BOOT: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;

/// The claim, and the setup sequence's first write after it.
fn claim(bar: &Arc<FakeBar>, resets: &Arc<AtomicU32>) {
    set_command(BOOT);
    watch(probe(bar, resets));
    command(1, 1, false).expect("the claim-time write is taken");
}

#[test]
fn bus_mastering_goes_on_only_after_the_reset_and_the_rings() {
    let _turn = entropy(true);
    let (bar, resets) = (window(), Arc::new(AtomicU32::new(0)));
    let _part = live(&bar, counting(&resets));
    claim(&bar, &resets);
    finish(Memory::new().driver(&bar)).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets.load(Ordering::SeqCst));
}

#[test]
fn a_driver_torn_down_stops_and_resets_the_part_and_clears_bus_master_first() {
    let _turn = entropy(true);
    let (bar, resets) = (window(), Arc::new(AtomicU32::new(0)));
    let _part = live(&bar, counting(&resets));
    claim(&bar, &resets);
    let mut mem = Memory::new();
    let driver = finish(mem.driver(&bar)).expect("brought up");
    let (mark, done) = (seen().len(), resets.load(Ordering::SeqCst));
    driver.release();
    released_quiet(&seen()[mark..], done + 1);
}
