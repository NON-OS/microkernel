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

//! The Immediate Command status register as a controller keeps it (HDA 1.0a
//! 3.4.3): ICB set by a post and cleared with the answer in ICII, IRV set
//! with it and cleared only by the driver writing it as 1. A bare window
//! reads back what was written, so a post writing IRV as 0 would clear it.
//! The model shows ICV (bit 2, read-only) beside the state, so a value
//! without it is the driver's write. Stepped from the driver's own polls
//! (`clock::on_poll`), it acts between a write and the read that follows.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::constants::{ICII, ICIS, ICIS_ICB, ICIS_IRV};

const ICV: u16 = 1 << 2;

pub struct Immediate {
    bar: Arc<FakeBar>,
    answer: u32,
    busy: bool,
    valid: bool,
    shown: u16,
}

impl Immediate {
    /// A controller answering every post with `answer`, and with `stale`
    /// already in ICII and IRV set when given: the answer to a verb the
    /// driver stopped waiting for.
    pub fn new(bar: &Arc<FakeBar>, answer: u32, stale: Option<u32>) -> Self {
        if let Some(old) = stale {
            bar.present32(ICII as usize, old);
        }
        let valid = stale.is_some();
        let mut m = Immediate { bar: Arc::clone(bar), answer, busy: false, valid, shown: 0 };
        m.show();
        m
    }

    /// One step: a write taken as the register takes it, else an answer
    /// for a post seen on the step before.
    pub fn step(&mut self) {
        let cell = self.bar.wrote16(ICIS as usize);
        if cell != self.shown {
            if cell & ICIS_IRV != 0 {
                self.valid = false;
            }
            self.busy |= cell & ICIS_ICB != 0;
        } else if self.busy {
            self.bar.present32(ICII as usize, self.answer);
            (self.busy, self.valid) = (false, true);
        }
        self.show();
    }

    fn show(&mut self) {
        self.shown = ICV | (u16::from(self.busy) * ICIS_ICB) | (u16::from(self.valid) * ICIS_IRV);
        self.bar.present16(ICIS as usize, self.shown);
    }
}
