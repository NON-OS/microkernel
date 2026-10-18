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

//! One step of a wait past the spin budget: blocked on the interrupt of
//! the ring it reads when one is bound, and once the kernel refuses that,
//! or with none, a bounded stretch of yields and then sleeps.

use nonos_libc::{irq_waited_on, slept_ms, yields};

use crate::controller::park::{IrqWait, SPIN_BUDGET};
use nonos_libc::wait::YIELD_BUDGET;

#[test]
fn a_wait_past_both_budgets_with_no_interrupt_sleeps_instead_of_yielding() {
    let (yielded, slept) = (yields(), slept_ms());
    IrqWait::new(0).park(SPIN_BUDGET + YIELD_BUDGET + 1);
    assert_eq!(yields(), yielded, "no yield once the budget is spent");
    assert!(slept_ms() > slept, "the poll slept");
}

#[test]
fn a_wait_inside_the_yield_budget_yields_once_and_does_not_sleep() {
    let (yielded, slept) = (yields(), slept_ms());
    IrqWait::new(0).park(SPIN_BUDGET + 1);
    assert_eq!(yields(), yielded + 1);
    assert_eq!(slept_ms(), slept);
}

#[test]
fn a_wait_blocks_on_its_own_grant_and_polls_once_the_kernel_refuses_it() {
    let (mut a, mut b) = (IrqWait::new(0x141), IrqWait::new(0x241));
    let slept = slept_ms();
    a.park(SPIN_BUDGET + 1);
    assert_eq!(irq_waited_on(), 0x141);
    assert!(slept_ms() > slept, "a refused wait sleeps its step");
    b.park(SPIN_BUDGET + 1);
    assert_eq!(irq_waited_on(), 0x241, "the other ring's wait is its own");
    let yielded = yields();
    a.park(SPIN_BUDGET + 1);
    assert_eq!(yields(), yielded + 1, "refused once, it polls from then on");
}
