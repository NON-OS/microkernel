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

//! A clock that moves only when the driver reads it. Every wait the driver
//! makes reads the clock as it polls, so a test that plays the controller
//! acts on each read (`on_tick`), and a 30 s budget runs out in microseconds.

use std::cell::{Cell, RefCell};

type Tick = Box<dyn FnMut()>;

thread_local! {
    static NOW: Cell<i64> = const { Cell::new(0) };
    static STEP: Cell<i64> = const { Cell::new(1) };
    static TICK: RefCell<Option<Tick>> = const { RefCell::new(None) };
    static FAILING: Cell<bool> = const { Cell::new(false) };
}

/// While `on`, every read fails (returns -1), as the kernel's can; time still
/// passes, and the hook still runs.
pub fn fail_clock(on: bool) {
    FAILING.with(|f| f.set(on));
}

/// Each later read moves the clock on by `step_ms`.
pub fn set_clock(step_ms: i64) {
    STEP.with(|s| s.set(step_ms));
}

/// Run `f` on every clock read, before the time is returned.
pub fn on_tick(f: impl FnMut() + 'static) {
    TICK.with(|t| *t.borrow_mut() = Some(Box::new(f)));
}

pub fn mk_uptime_ms() -> i64 {
    // Taken out while it runs, so a hook that reads the clock cannot borrow
    // itself twice; put back unless it set a new one.
    let hook = TICK.with(|t| t.borrow_mut().take());
    if let Some(mut f) = hook {
        f();
        TICK.with(|t| {
            let mut slot = t.borrow_mut();
            if slot.is_none() {
                *slot = Some(f);
            }
        });
    }
    let now = NOW.with(|n| n.get());
    NOW.with(|n| n.set(now + STEP.with(|s| s.get())));
    if FAILING.with(Cell::get) {
        -1
    } else {
        now
    }
}
