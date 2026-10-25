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

//! A model stepped from the driver's own polls: each read of the clock on
//! the thread that set a step runs it first. What the step shows lands
//! between the driver's write and its next read of the register, where a
//! model on its own thread lands only when the scheduler lets it.

use std::cell::RefCell;

type Step = Box<dyn FnMut()>;

thread_local! {
    static STEP: RefCell<Option<Step>> = const { RefCell::new(None) };
}

/// Run `step` on each clock read on this thread until the guard drops.
pub fn on_poll(step: impl FnMut() + 'static) -> Polled {
    STEP.with(|s| *s.borrow_mut() = Some(Box::new(step)));
    Polled(())
}

/// A step in place; dropping it ends the stepping.
#[must_use = "the step ends when this drops"]
pub struct Polled(());

impl Drop for Polled {
    fn drop(&mut self) {
        STEP.with(|s| *s.borrow_mut() = None);
    }
}

pub(super) fn step() {
    STEP.with(|s| {
        if let Some(f) = s.borrow_mut().as_mut() {
            f();
        }
    });
}
