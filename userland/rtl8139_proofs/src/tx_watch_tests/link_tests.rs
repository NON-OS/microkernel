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

//! The watchdog reads the link only when the answer turns on it: a frame
//! finished, or none in flight, settles it without a port read, which every
//! send would otherwise pay for.

use std::{cell::Cell, rc::Rc};

use nonos_libc::{attach, entropy, Port};

use crate::constants::regs::REG_MSR;
use crate::init::finish;
use crate::part::{driver_over, Model};
use crate::server::tx_watch::check;

/// The part, counting the reads of the media status register.
struct Counting(Model, Rc<Cell<u32>>);

impl Port for Counting {
    fn read(&mut self, offset: u16, width: u8) -> Option<u32> {
        if offset == REG_MSR {
            self.1.set(self.1.get() + 1);
        }
        self.0.read(offset, width)
    }

    fn write(&mut self, offset: u16, width: u8, value: u32) -> bool {
        self.0.write(offset, width, value)
    }
}

#[test]
fn the_link_is_read_only_while_frames_are_in_flight_and_none_finished() {
    let _turn = entropy(true);
    let reads = Rc::new(Cell::new(0));
    let model =
        Model { regs: [0; 256], stuck_in_reset: false, refuse: None, resets: Default::default() };
    let _port = attach(Box::new(Counting(model, Rc::clone(&reads))));
    let mut d = finish(driver_over(0)).expect("brought up");
    let before = reads.get();
    d.tx_cur = 2;
    assert!(!check(&mut d, true), "a finished frame is a sign of life");
    d.tx_cur = d.tx_dirty;
    assert!(!check(&mut d, false), "nothing in flight");
    assert_eq!(reads.get(), before, "the link was not read");
    d.tx_cur = 2;
    assert!(!check(&mut d, false), "in flight, inside the bound");
    assert_eq!(reads.get(), before + 1, "read once it decides");
}
