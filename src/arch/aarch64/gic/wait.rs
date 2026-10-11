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

//! The one way the GIC code waits on hardware: against the generic counter,
//! with a budget, so a frame that never answers costs a refused boot line
//! instead of a hung CPU.

use crate::arch::aarch64::timer::generic::{current_count, milliseconds_to_ticks};

/// Long enough for any real distributor or redistributor to settle; the
/// architecture gives no bound, so this is a ceiling on patience.
pub(super) const BUDGET_MS: u64 = 10;

/// Poll `done` until it holds or `BUDGET_MS` has passed. A counter with no
/// frequency gives a zero budget, so the condition gets one last look and
/// the wait fails closed rather than spinning on a clock that never moves.
pub(super) fn until(mut done: impl FnMut() -> bool) -> bool {
    let deadline = current_count().saturating_add(milliseconds_to_ticks(BUDGET_MS));
    while !done() {
        if current_count() >= deadline {
            return done();
        }
        core::hint::spin_loop();
    }
    true
}
