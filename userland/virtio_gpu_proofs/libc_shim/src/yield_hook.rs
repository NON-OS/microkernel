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
//! The queue wait's yield, with a test's device step hung on it.

use std::cell::RefCell;

type Hook = Box<dyn FnMut()>;

thread_local! {
    static HOOK: RefCell<Option<Hook>> = const { RefCell::new(None) };
}

/// Run `step` on every yield of this thread until `off_yield`. It may
/// panic, as a driver killed while it waited.
pub fn on_yield(step: impl FnMut() + 'static) {
    HOOK.with(|h| *h.borrow_mut() = Some(Box::new(step)));
}

pub fn off_yield() {
    HOOK.with(|h| *h.borrow_mut() = None);
}

pub fn mk_yield() -> i64 {
    // Taken out while it runs, so a panic in it leaves no borrow behind; a
    // step that panicked is not put back, the driver being dead.
    let hook = HOOK.with(|h| h.borrow_mut().take());
    if let Some(mut step) = hook {
        step();
        HOOK.with(|h| {
            let mut h = h.borrow_mut();
            if h.is_none() {
                *h = Some(step);
            }
        });
    }
    0
}
