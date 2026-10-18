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

//! The loop's sleep when every controller's line is bound: on the
//! interrupts themselves, briefly enough that requests are still answered.

use crate::bank::Bank;
use nonos_libc::mk_irq_wait;

/// Longest the loop sleeps on interrupts before it looks for requests.
const IRQ_WAIT_MS: u64 = 10;

/// With every bank bound, sleeps until one fires or `IRQ_WAIT_MS` passes.
pub fn wait(banks: &[Bank], last: &mut u64) {
    if !all_bound(banks) {
        return;
    }
    let mut seq = *last;
    if mk_irq_wait(0, *last, IRQ_WAIT_MS, &mut seq) >= 0 {
        *last = seq;
    }
}

/// Every bank bound: requests need only a glance between interrupts.
pub fn all_bound(banks: &[Bank]) -> bool {
    !banks.is_empty() && banks.iter().all(|b| b.grant != 0)
}
