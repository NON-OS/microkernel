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

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{entropy, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::order::{bus_master_after_reset_and_rings, no_reset_while_mastering, released_quiet};
use super::probe::{probe, resets};
use crate::constants::Family;
use crate::init::finish::finish;
use crate::model::live::{live, Live};
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{i82574, phy_at};
use crate::setup::{command, Driver};

/// An 82574, as the kernel's boot pass leaves it (Bus Master on), claimed,
/// and given the setup sequence's first write.
pub fn claimed() -> (Arc<FakeBar>, Live) {
    let bar = i82574();
    let part = live(&bar, phy_at(1), Behaviour::default());
    set_command(MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER);
    watch(probe(&bar, &part.how));
    command(9, 1, false).expect("the claim-time write is taken");
    (bar, part)
}

pub fn up(bar: &FakeBar, mem: &mut Memory) -> Result<Driver, &'static str> {
    finish(mem.driver(bar, Family::I82574, 0x10D3))
}

#[test]
fn bus_mastering_goes_on_only_after_the_reset_and_the_rings() {
    let _turn = entropy(true);
    let (bar, part) = claimed();
    up(&bar, &mut Memory::new()).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets(&part.how));
}

#[test]
fn a_driver_torn_down_stops_and_resets_the_part_and_clears_bus_master_first() {
    let _turn = entropy(true);
    let (bar, part) = claimed();
    let mut mem = Memory::new();
    let d = up(&bar, &mut mem).expect("brought up");
    let (mark, done) = (seen().len(), resets(&part.how));
    d.release();
    released_quiet(&seen()[mark..], done + 1);
    no_reset_while_mastering(&seen(), resets(&part.how));
}
