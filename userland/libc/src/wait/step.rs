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

//! Which kind of wait one poll makes. Pure, so the policy is proven apart
//! from the syscalls that carry it out.

// Polls burned before the first park. A device usually writes its completion
// while the doorbell store is still in flight, so one that answers never
// reaches the budget and never pays a syscall.
pub const SPIN_BUDGET: u32 = 1024;
// With no interrupt to block on, polls that yield before the wait starts to
// sleep. A device that answers within a few scheduler passes is not slowed by
// a sleep; one that does not answer costs a timed sleep per poll from here on,
// not a core spinning on yields that return at once on an idle machine.
pub const YIELD_BUDGET: u32 = 1024;
// Ceiling on a single park. The interrupt wakes the capsule the moment the
// event lands, so this only bounds the path where no interrupt arrives at
// all; a device that never answers then costs one wakeup per scheduler tick
// instead of a whole core.
pub const PARK_MS: u64 = 2;

/// What one poll of a completion wait does, by how many polls came before it
/// and whether an interrupt is bound to block on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// A tight spin: the event is usually already there.
    Spin,
    /// Block on the device interrupt, at most `PARK_MS`.
    Wait,
    /// No interrupt: give the processor away once.
    Yield,
    /// No interrupt and still nothing: sleep `PARK_MS`.
    Sleep,
}

pub const fn step(spins: u32, has_irq: bool) -> Step {
    if spins <= SPIN_BUDGET {
        Step::Spin
    } else if has_irq {
        Step::Wait
    } else if spins <= SPIN_BUDGET.saturating_add(YIELD_BUDGET) {
        Step::Yield
    } else {
        Step::Sleep
    }
}
