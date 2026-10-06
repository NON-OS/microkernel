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

//! A completion wait parks: with the controller interrupt it blocks on it,
//! and without one it yields only for a bounded stretch and then sleeps, so
//! a controller that stops answering costs wakeups, never a whole core.

use nonos_libc::{slept_ms, yields};

use crate::controller::park::{park_step, step, Step, SPIN_BUDGET, YIELD_BUDGET};

#[test]
fn the_first_polls_spin_whether_or_not_an_interrupt_is_bound() {
    for spins in [0, 1, SPIN_BUDGET / 2, SPIN_BUDGET] {
        assert_eq!(step(spins, true), Step::Spin);
        assert_eq!(step(spins, false), Step::Spin);
    }
}

#[test]
fn with_an_interrupt_every_later_poll_blocks_on_it() {
    for spins in [SPIN_BUDGET + 1, SPIN_BUDGET + YIELD_BUDGET + 1, u32::MAX] {
        assert_eq!(step(spins, true), Step::Wait);
    }
}

#[test]
fn without_an_interrupt_the_yields_are_bounded_and_then_every_poll_sleeps() {
    assert_eq!(step(SPIN_BUDGET + 1, false), Step::Yield);
    assert_eq!(step(SPIN_BUDGET + YIELD_BUDGET, false), Step::Yield);
    assert_eq!(step(SPIN_BUDGET + YIELD_BUDGET + 1, false), Step::Sleep);
    assert_eq!(step(u32::MAX, false), Step::Sleep, "a saturated count still sleeps");
    let yielding = (0..=SPIN_BUDGET + 4 * YIELD_BUDGET)
        .filter(|&s| step(s, false) == Step::Yield)
        .count();
    assert_eq!(yielding as u32, YIELD_BUDGET, "exactly one bounded stretch of yields");
}

#[test]
fn the_steps_never_go_back_from_sleeping_to_spinning() {
    let order = |s: Step| match s {
        Step::Spin => 0,
        Step::Yield | Step::Wait => 1,
        Step::Sleep => 2,
    };
    let mut last = 0;
    for spins in 0..=SPIN_BUDGET + 2 * YIELD_BUDGET {
        let now = order(step(spins, false));
        assert!(now >= last, "poll {spins} went back to a cheaper wait");
        last = now;
    }
}

#[test]
fn a_wait_past_both_budgets_with_no_interrupt_sleeps_instead_of_yielding() {
    // No test sets the interrupt grant, so the shipping park_step polls
    // without one, as on a machine whose MSI-X bind was refused.
    let (yielded, slept) = (yields(), slept_ms());
    park_step(SPIN_BUDGET + YIELD_BUDGET + 1);
    assert_eq!(yields(), yielded, "no yield once the budget is spent");
    assert!(slept_ms() > slept, "the poll slept");
}

#[test]
fn a_wait_inside_the_yield_budget_yields_once_and_does_not_sleep() {
    let (yielded, slept) = (yields(), slept_ms());
    park_step(SPIN_BUDGET + 1);
    assert_eq!(yields(), yielded + 1);
    assert_eq!(slept_ms(), slept);
}
