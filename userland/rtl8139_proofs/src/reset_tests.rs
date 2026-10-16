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

//! The software reset on the clock: a part that never leaves reset is given
//! up on once the bound has passed, not after a count of reads; one that
//! leaves an engine on is named and never given Bus Master.

use std::time::Instant;
use std::{cell::Cell, rc::Rc};

use nonos_libc::{attach, entropy, seen, set_command, watch, Port};

use crate::constants::regs::{CMD_RX_ENABLE, REG_CMD};
use crate::init::{finish, reset_part};
use crate::part::{driver_over, part, Model};
use crate::pio::Pio;
use crate::probe::probe;

#[test]
fn a_part_stuck_in_reset_is_given_up_on_by_the_clock() {
    let _part = part(true);
    let start = Instant::now();
    assert_eq!(reset_part(&Pio::new(3)), Err("rtl8139 reset timeout"));
    assert!(start.elapsed().as_millis() >= 10, "waited the 10 ms bound");
}

/// The part, with its receiver reading on whatever was written.
struct Sticky(Model);

impl Port for Sticky {
    fn read(&mut self, offset: u16, width: u8) -> Option<u32> {
        let v = self.0.read(offset, width)?;
        Some(if offset == REG_CMD { v | CMD_RX_ENABLE as u32 } else { v })
    }

    fn write(&mut self, offset: u16, width: u8, value: u32) -> bool {
        self.0.write(offset, width, value)
    }
}

#[test]
fn a_receiver_the_reset_leaves_on_is_named_and_never_mastered() {
    let _turn = entropy(true);
    let resets = Rc::new(Cell::new(0));
    let model = Model { regs: [0; 256], stuck_in_reset: false, refuse: None, resets };
    let _port = attach(Box::new(Sticky(model)));
    set_command(0);
    watch(probe(&Rc::new(Cell::new(0))));
    let err = finish(driver_over(0)).err();
    assert_eq!(err, Some("rtl8139 engines still on after reset"));
    assert!(seen().iter().all(|s| !s.bus_master), "{:?}", seen());
}
