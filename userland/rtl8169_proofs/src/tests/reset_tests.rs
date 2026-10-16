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

//! Software reset, against a part that completes it, one that does not, and
//! one that leaves its receiver on, which is named and never given Bus
//! Master.

use std::sync::{atomic::AtomicU32, Arc};

use nonos_devmodel::FakeBar;
use nonos_libc::{entropy, seen, set_command, watch};

use super::memory::{Memory, CHIP};
use super::model::{live, resetting_part, window};
use super::probe::probe;
use crate::constants::regs::{CMD_RESET, CMD_RX_ENABLE, REG_CMD};
use crate::init::{finish, reset_run};
use crate::regs::Regs;

#[test]
fn reset_is_requested_and_waited_for() {
    let bar = window();
    let _part = live(&bar, resetting_part);
    reset_run(&Regs::new(bar.base()), CHIP.ver).expect("a conforming part completes reset");
    assert_eq!(bar.wrote8(REG_CMD) & CMD_RESET, 0);
}

#[test]
fn a_part_that_never_completes_reset_is_given_up_on() {
    let bar = window();
    assert!(reset_run(&Regs::new(bar.base()), CHIP.ver).is_err());
    assert_ne!(bar.wrote8(REG_CMD) & CMD_RESET, 0, "the request was made");
}

/// A part that completes its reset with the receiver reading on.
fn sticky(b: &FakeBar) {
    let cmd = b.wrote8(REG_CMD);
    if cmd & CMD_RESET != 0 {
        b.present8(REG_CMD, (cmd & !CMD_RESET) | CMD_RX_ENABLE);
    }
}

#[test]
fn a_receiver_the_reset_leaves_on_is_named_and_never_mastered() {
    let _turn = entropy(true);
    let bar = window();
    let _part = live(&bar, sticky);
    set_command(0);
    watch(probe(&bar, &Arc::new(AtomicU32::new(0))));
    let err = finish(Memory::new().driver(&bar)).err();
    assert_eq!(err, Some("rtl8169 engines still on after reset"));
    assert!(seen().iter().all(|s| !s.bus_master), "{:?}", seen());
}
