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

//! The service loop: take a trap from any process or thread the family
//! hosts, answer it or leave the caller parked, and end what has exited.

use nonos_libc::{mk_foreign_wait, ForeignFrame};

use super::family::Family;
use crate::linux::guest::Guest;

/// How long one wait blocks before looking at the guest again.
const WAIT_MS: u64 = 250;

pub fn serve(guest: Guest) -> i32 {
    let mut family = Family::new(guest);
    loop {
        let mut frame = ForeignFrame::default();
        // A sleeper or a parked wait due sooner than the usual wait shortens it.
        let wait = family.next_wake_ms().map_or(WAIT_MS, |ms| ms.clamp(1, WAIT_MS));
        if mk_foreign_wait(&mut frame, wait) > 0 {
            family.answer(&frame);
        }
        family.settle_sleeps();
        family.settle_futex();
        family.settle_waits();
        family.reap();
        if let Some(code) = family.done() {
            super::tally::report();
            return code;
        }
    }
}
