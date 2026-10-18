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

//! A bound interrupt grant. No interrupt path exists on the host: a wait on
//! a grant bound on this thread runs to its timeout, and any other is
//! refused, as a kernel that refused the grant would. The grant waited on
//! is remembered per thread, so a test can say whose interrupt a wait
//! blocked on, and an unbind is recorded.

use std::cell::{Cell, RefCell};
use std::time::Duration;

use crate::log::{record, Call};

thread_local! {
    static WAITED_ON: Cell<u64> = const { Cell::new(0) };
    static LIVE: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn keep_bound(grant: u64) {
    LIVE.with(|l| l.borrow_mut().push(grant));
}

fn is_bound(grant: u64) -> bool {
    LIVE.with(|l| l.borrow().contains(&grant))
}

/// The grant this thread last waited on, 0 if none.
pub fn irq_waited_on() -> u64 {
    WAITED_ON.with(|c| c.get())
}

pub extern "C" fn mk_irq_wait(grant: u64, _last_seq: u64, timeout_ms: u64, _out: *mut u64) -> i64 {
    WAITED_ON.with(|c| c.set(grant));
    if !is_bound(grant) {
        return -1;
    }
    std::thread::sleep(Duration::from_millis(timeout_ms));
    0
}

pub fn mk_irq_unbind(grant_id: u64) -> i64 {
    LIVE.with(|l| l.borrow_mut().retain(|&g| g != grant_id));
    record(Call::IrqUnbind(grant_id));
    0
}

pub fn mk_irq_ack(_grant_id: u64) -> i64 {
    0
}
