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

//! A scripted interrupt park over the shared model: each sleep lets one
//! frame pass (unless the pipe is stopped) and returns the message the
//! model raised, if the machine delivers it.

use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::park::{Park, Woke};
use crate::model::Gen9;

pub struct Blanks {
    pub model: Rc<RefCell<Gen9>>,
    /// Whether messages reach the driver on this machine.
    pub delivers: bool,
    /// The pipe gives no blank at all (unplugged mid-wait).
    pub stopped: bool,
    pub refused: bool,
    /// Unrelated wakes to give before any other.
    pub early: u32,
    pub waits: Vec<u64>,
    pub acks: u32,
    seen: u32,
}

impl Blanks {
    pub fn new(model: Rc<RefCell<Gen9>>) -> Self {
        let (delivers, stopped, refused, early) = (true, false, false, 0);
        Blanks { model, delivers, stopped, refused, early, waits: Vec::new(), acks: 0, seen: 0 }
    }
}

impl Park for Blanks {
    fn bound(&self) -> bool {
        true
    }

    fn wait(&mut self, ms: u64) -> Woke {
        self.waits.push(ms);
        if self.refused {
            return Woke::Refused;
        }
        if self.early > 0 {
            self.early -= 1;
            return Woke::Early;
        }
        let mut m = self.model.borrow_mut();
        if m.msi.sent == self.seen && !self.stopped {
            m.vblank();
        }
        let got = m.msi.sent > self.seen;
        self.seen = m.msi.sent;
        match got && self.delivers {
            true => Woke::Irq,
            false => Woke::TimedOut,
        }
    }

    fn ack(&mut self) {
        self.acks += 1;
    }
}
