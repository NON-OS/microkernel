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

//! An I219 claimed with descriptors pending. They were fetched under a
//! previous owner, a dead capsule or firmware, and may name frames since
//! freed, so Bus Master never goes on for a flush: the part is reset with
//! Bus Master off and comes up on this driver's rings, or, when that reset
//! does not complete, the attempt ends by name and the device goes back,
//! Bus Master never on.

use std::sync::atomic::Ordering::SeqCst;

use nonos_libc::{command, entropy, logged, seen, set_command, watch, Seen, Step};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use super::flush_master_tests::pending;
use super::order::{bus_master_after_reset_and_rings, released_quiet};
use super::probe::{probe, resets};
use crate::constants::Family;
use crate::init::finish::finish;
use crate::init::reset_part::DESCRIPTORS_PENDING;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at};

const LEFT: &str =
    "e1000e: descriptor rings left pending by a previous owner, reset with Bus Master off";

#[test]
fn an_i219_claimed_with_descriptors_pending_is_reset_with_bus_master_off_not_flushed() {
    let _turn = entropy(true);
    let bar = pch();
    let part = live(&bar, phy_at(2), Behaviour::default());
    pending(&bar);
    // Bus Master as the boot pass left it: the broker ORs the claim's write
    // in today (setup/pci.rs), so the bring-up meets it on.
    set_command(MEM | BM);
    watch(probe(&bar, &part.how));
    let mut mem = Memory::new();
    finish(mem.driver(&bar, Family::PchCnp, 0x15BB)).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets(&part.how));
    assert_eq!(mem.tx[0].length, 0, "no flush descriptor handed over");
    assert!(logged().iter().any(|l| l == LEFT), "{:?}", logged());
}

#[test]
fn a_reset_that_does_not_complete_over_them_is_named_and_the_device_goes_back() {
    let _turn = entropy(true);
    let bar = pch();
    let part = live(&bar, phy_at(2), Behaviour::default());
    part.how.reset_stuck.store(true, SeqCst);
    pending(&bar);
    set_command(MEM | BM);
    watch(probe(&bar, &part.how));
    let err = finish(Memory::new().driver(&bar, Family::PchCnp, 0x15BB)).err();
    assert_eq!(err, Some(DESCRIPTORS_PENDING));
    let on = |s: &Seen| matches!(s.step, Step::Command(v) if v & BM != 0);
    assert!(!seen().iter().any(on) && command() & BM == 0, "{:?}", seen());
    released_quiet(&seen(), 0);
}
