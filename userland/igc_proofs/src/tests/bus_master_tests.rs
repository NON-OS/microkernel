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

use std::sync::atomic::{AtomicU32, Ordering::SeqCst};
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{entropy, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::memory::Memory;
use super::model::{live, window, Live};
use super::order::{bus_master_after_reset_and_rings, released_quiet};
use super::phy_model::{bmcr, phy, BMCR_POWERED_DOWN};
use super::probe::{counting, probe};
use crate::init::finish;
use crate::setup::command;

/// As the kernel's boot pass leaves every endpoint, and the claim with it.
const BOOT: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;

/// A part that resets, counted, and answers MDIO, claimed and given the
/// setup sequence's first write; `also` runs on every look.
pub fn claimed(also: fn(&FakeBar)) -> (Arc<FakeBar>, Live, Arc<AtomicU32>) {
    let (bar, resets) = (window(), Arc::new(AtomicU32::new(0)));
    let mdio = phy(bmcr(BMCR_POWERED_DOWN));
    let live = live(&bar, counting(&resets, move |b| (mdio(b), also(b)).1));
    set_command(BOOT);
    watch(probe(&bar, &resets));
    command(9, 1, false).expect("the claim-time write is taken");
    (bar, live, resets)
}

#[test]
fn bus_mastering_goes_on_only_after_the_reset_and_the_rings() {
    let _turn = entropy(true);
    let (bar, _part, resets) = claimed(|_| {});
    finish(Memory::new().driver(&bar)).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets.load(SeqCst));
}

#[test]
fn a_driver_torn_down_stops_and_resets_the_part_and_clears_bus_master_first() {
    let _turn = entropy(true);
    let (bar, _part, resets) = claimed(|_| {});
    let mut mem = Memory::new();
    let driver = finish(mem.driver(&bar)).expect("brought up");
    let (mark, done) = (seen().len(), resets.load(SeqCst));
    driver.release();
    released_quiet(&seen()[mark..], done + 1);
}
