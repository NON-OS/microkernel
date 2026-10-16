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

//! A test's part acting on the driver's looks at the clock, as igc_proofs'
//! does. A wait takes its deadline in its first look, so a part run here
//! can place that deadline in a millisecond of uptime, then hold a later
//! look until uptime reaches a chosen count: what a host that schedules the
//! driver's thread as it likes cannot be relied on to do.

use std::cell::RefCell;

type Part = Box<dyn Fn()>;

thread_local! {
    static PART: RefCell<Option<Part>> = const { RefCell::new(None) };
}

/// The part acting on this thread's looks, until it is dropped.
pub struct OnClock;

impl Drop for OnClock {
    fn drop(&mut self) {
        PART.with(|p| p.borrow_mut().take());
    }
}

/// Run `f` each time this thread reads the clock, before the read.
pub fn on_clock(f: impl Fn() + 'static) -> OnClock {
    PART.with(|p| *p.borrow_mut() = Some(Box::new(f)));
    OnClock
}

pub(crate) fn look() {
    PART.with(|p| p.borrow().as_ref().map_or((), |f| f()));
}
