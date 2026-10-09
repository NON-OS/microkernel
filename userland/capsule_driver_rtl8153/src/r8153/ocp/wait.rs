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

//! Waiting for the chip on the clock. Linux counts passes of a loop with a
//! sleep in each (500 of 20 ms for the autoload, 1000 of 1 to 2 ms for the
//! link list); the same budgets here are milliseconds of uptime, so a
//! wait lasts as long whatever the core count.

use nonos_libc::{mk_idle_ms, Deadline};
use nonos_usbnet::Bus;

use super::dev::Dev;
use crate::r8153::fail::E_TIMEDOUT;

/// Asks `done` until it answers true, sleeping `pause_ms` between asks,
/// for at most `budget_ms`; a register error ends the wait at once.
pub fn wait_until<B: Bus>(
    dev: &mut Dev<B>,
    budget_ms: u64,
    pause_ms: u64,
    mut done: impl FnMut(&mut Dev<B>) -> Result<bool, i32>,
) -> Result<(), i32> {
    let deadline = Deadline::after_ms(budget_ms);
    loop {
        if done(dev)? {
            return Ok(());
        }
        if deadline.expired() {
            return Err(E_TIMEDOUT);
        }
        let _ = mk_idle_ms(pause_ms);
    }
}
