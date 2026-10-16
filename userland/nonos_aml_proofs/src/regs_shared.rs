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

//! The modelled registers behind a shared handle.

use crate::events::EventIo;
use crate::regs::Regs;

/// The registers shared with a GPE method's stand-in, so it can look at
/// them while it runs.
#[derive(Clone, Default)]
pub struct Shared(pub std::rc::Rc<std::cell::RefCell<Regs>>);

impl EventIo for Shared {
    fn read(&mut self, port: u16, width: u8) -> Option<u16> {
        self.0.borrow_mut().read(port, width)
    }

    fn write(&mut self, port: u16, width: u8, value: u16) -> bool {
        self.0.borrow_mut().write(port, width, value)
    }

    fn sleep(&mut self, ms: u64) {
        self.0.borrow_mut().sleep(ms)
    }

    fn now_ms(&mut self) -> u64 {
        self.0.borrow().now
    }
}
