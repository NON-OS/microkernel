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

//! The part behind the port grant. It is reached only through `mk_pio_read`
//! and `mk_pio_write`, so a model behind those two calls answers in the
//! test's own thread.

use std::cell::RefCell;

/// A part on the other side of the port grant. `read` answers `None` and
/// `write` answers `false` to refuse the call the way the broker would.
pub trait Port {
    fn read(&mut self, offset: u16, width: u8) -> Option<u32>;
    fn write(&mut self, offset: u16, width: u8, value: u32) -> bool;
}

thread_local! {
    static PORT: RefCell<Option<Box<dyn Port>>> = const { RefCell::new(None) };
}

/// Detaches the model when dropped, so a later test on the same thread
/// starts with nothing behind the port.
pub struct Attached;

impl Drop for Attached {
    fn drop(&mut self) {
        PORT.with(|p| *p.borrow_mut() = None);
    }
}

pub fn attach(port: Box<dyn Port>) -> Attached {
    PORT.with(|p| *p.borrow_mut() = Some(port));
    Attached
}

pub fn mk_pio_read(_grant: u64, offset: u16, width: u8, out: *mut u32) -> i64 {
    let answer = PORT.with(|p| p.borrow_mut().as_mut().and_then(|port| port.read(offset, width)));
    match answer {
        Some(value) => {
            /*
             * SAFETY: the driver passes a reference to a live u32, as the
             * real call requires.
             */
            unsafe { *out = value };
            0
        }
        None => -1,
    }
}

pub fn mk_pio_write(_grant: u64, offset: u16, width: u8, value: u32) -> i64 {
    let taken =
        PORT.with(|p| p.borrow_mut().as_mut().is_some_and(|port| port.write(offset, width, value)));
    if taken {
        0
    } else {
        -1
    }
}
