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

//! The driver on the legacy bench, and the server's turns: a request, then `settle`.

use std::panic::{catch_unwind, AssertUnwindSafe};

use nonos_libc::{advance, events, offer_irq, Event};

use super::bench::bench;
use crate::io::{submit, BlkError};
use crate::server::settle::settle;
use crate::server::streak::{Streak, SERVED_SPAN_MS};
use crate::setup::{self, Driver};

pub struct Up {
    pub dev: super::legacy_dev::Dev,
    pub driver: Driver,
    pub streak: Streak,
}

/// A device on INTx that answers nothing, brought up.
pub fn up() -> Up {
    let dev = bench(8, 2048, 1);
    offer_irq(false, true);
    Up { dev, driver: setup::run().expect("the device comes up"), streak: Streak::default() }
}

pub fn claims() -> u32 {
    events().iter().filter(|e| **e == Event::Claim).count() as u32
}

impl Up {
    pub fn read(&mut self) -> Result<(), BlkError> {
        let d = &mut self.driver;
        submit(d.transport, &mut d.queue, d.irq_grant, crate::queue::Direction::Read, 0, 1)
    }

    /// `n` turns: the code the driver exited with, if it did.
    pub fn turns(&mut self, n: u32) -> Option<String> {
        let run = catch_unwind(AssertUnwindSafe(|| {
            for _ in 0..n {
                let _ = self.read();
                settle(&mut self.driver, &mut self.streak);
            }
        }));
        run.err().map(|end| end.downcast_ref::<String>().cloned().unwrap_or_default())
    }

    /// The device answers reads, with `status` and one byte, a quarter span apart up to `ms`.
    pub fn answers_for(&mut self, ms: u64, status: u8) {
        self.dev.borrow_mut().answers = Some((1, status));
        let mut was = 0;
        for at in (0..ms).step_by(SERVED_SPAN_MS as usize / 4).chain([ms]) {
            advance(at - was);
            was = at;
            assert_eq!(self.read().is_ok(), status == 0, "a legacy read answered");
            settle(&mut self.driver, &mut self.streak);
        }
        self.dev.borrow_mut().answers = None;
    }
}
