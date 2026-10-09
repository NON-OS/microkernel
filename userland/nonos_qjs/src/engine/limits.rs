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

//! How much time, heap and stack a page's code may take.

use super::ffi::{njs_set_budget, njs_set_clocks, njs_set_limits, njs_take_stopped};
use super::lifecycle::Engine;

/// What an engine is bounded by.
#[derive(Clone, Copy)]
pub struct Limits {
    /// Bytes of heap the runtime may hold; past it an allocation throws.
    pub memory: usize,
    /// Bytes of native stack below the frame the engine is entered from.
    pub stack: usize,
    /// Milliseconds one entry from the host (a script, a timer flush, an
    /// event dispatch, the load events) may run before it is stopped.
    pub budget_ms: u64,
    /// The host's monotonic clock in milliseconds. The engine has no
    /// system calls of its own, so the host lends it this one. The time
    /// budget is kept on it, and `performance.now()` reads it.
    pub clock: extern "C" fn() -> u64,
    /// The host's wall clock in Unix milliseconds, which `Date` reads and
    /// `Math.random` is seeded from.
    pub wall: extern "C" fn() -> i64,
}

/// Why the page's code was stopped.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stop {
    /// It ran past its time budget.
    Time,
    /// It asked for more heap than the limit.
    Memory,
}

/// What QuickJS throws when an allocation would pass the memory limit,
/// as the eval surface renders the exception.
pub(super) const OUT_OF_MEMORY: &str = "InternalError: out of memory";

impl Engine {
    /// A fresh runtime and context bounded by `limits`. None if the engine
    /// could not allocate.
    pub fn with_limits(limits: Limits) -> Option<Engine> {
        /* Lent before the context exists: making it seeds Math.random from
         * the wall clock and fixes where performance.now() counts from. */
        unsafe { njs_set_clocks(limits.clock, limits.wall) };
        let engine = Engine::new()?;
        unsafe { njs_set_limits(engine.rt, limits.memory, limits.stack, limits.budget_ms) };
        Some(engine)
    }

    /// The time budget for each entry from here on, the other limits as
    /// they were: a page's first run of its scripts may be given longer
    /// than the event handlers that follow.
    pub fn set_budget(&self, budget_ms: u64) {
        unsafe { njs_set_budget(budget_ms) };
    }

    /// Whether the page's code was stopped since this was last asked, and
    /// why. Reading clears it, so one stop is reported once.
    pub fn take_stop(&self) -> Option<Stop> {
        if unsafe { njs_take_stopped() } != 0 {
            self.oom.set(false);
            return Some(Stop::Time);
        }
        self.oom.replace(false).then_some(Stop::Memory)
    }
}
