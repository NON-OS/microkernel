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

//! The device's turn at the edges the driver cannot see past. A test sets
//! what the modelled reader does when the broker tears its grants down, so
//! a completion that arrives late lands where a real one would.

use std::cell::RefCell;
use std::rc::Rc;

type Act = Rc<dyn Fn()>;

thread_local! {
    static HOOK: RefCell<Option<Act>> = const { RefCell::new(None) };
}

/// What the device does at each such edge on this thread.
pub fn set_device_hook(act: impl Fn() + 'static) {
    HOOK.with(|h| *h.borrow_mut() = Some(Rc::new(act)));
}

/// Give the device its turn; nothing when no test set one.
pub fn device_acts() {
    let act = HOOK.with(|h| h.borrow().clone());
    if let Some(act) = act {
        act();
    }
}
