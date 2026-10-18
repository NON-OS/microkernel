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

//! The adapter answering a register write as it is made, per thread. The
//! proofs' `Regs` hands each 16-bit write here once the shipping code has
//! made it, so a model of an adapter that changes a register as it is
//! written, as one clamps a mode it cannot show, has acted before the
//! driver's next instruction: no second thread, and no race to lose.

use std::cell::RefCell;

type Hook = Box<dyn FnMut(u64, u16)>;

thread_local! {
    static ON_W16: RefCell<Option<Hook>> = const { RefCell::new(None) };
}

/// From now on, `hook(addr, value)` answers each 16-bit register write on
/// this thread, `addr` being the address written.
pub fn on_w16(hook: impl FnMut(u64, u16) + 'static) {
    ON_W16.with(|h| *h.borrow_mut() = Some(Box::new(hook)));
}

/// The driver wrote `value` at `addr`; the hook, when one is set, answers.
pub fn wrote16(addr: u64, value: u16) {
    ON_W16.with(|h| {
        if let Some(hook) = h.borrow_mut().as_mut() {
            hook(addr, value);
        }
    });
}
