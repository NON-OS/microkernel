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

//! One budget for everything a bind sends driver.xhci0. Binding runs on
//! the thread that answers net.core, so a device that answers slowly, or
//! never finishes a wait it is polled for, must not hold that thread. Each
//! call still waits out its own timeout, longer than the controller's
//! transfer wait, so no reply comes after its call gave up; once the
//! budget is spent no further call starts.

use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::mk_uptime_ms;

/// The longest a bind may spend before its next call is refused. The
/// slowest bring-up here, the RTL8153's, takes a few hundred milliseconds
/// on a healthy chip.
pub const BIND_MS: u64 = 15_000;
/// What a call made after the budget is spent ends with: ETIMEDOUT.
const E_TIMEDOUT: i32 = -110;

/// The uptime at which the bind under way runs out; 0 while none is.
static END_MS: AtomicU64 = AtomicU64::new(0);

/// Start a bind's budget of `ms`, or with `None` end it: the slot is given
/// back and the bound device served with no budget.
pub fn bind_budget(ms: Option<u64>) {
    let end = ms.map_or(0, |ms| now().saturating_add(ms).max(1));
    END_MS.store(end, Ordering::Relaxed);
}

/// `E_TIMEDOUT` once the bind under way has spent its budget.
pub(super) fn may_start() -> Result<(), i32> {
    match END_MS.load(Ordering::Relaxed) {
        0 => Ok(()),
        end if now() >= end => Err(E_TIMEDOUT),
        _ => Ok(()),
    }
}

fn now() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(0)
}
