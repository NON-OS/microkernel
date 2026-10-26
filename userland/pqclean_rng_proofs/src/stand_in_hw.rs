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

//! The hardware around the mounted files: an entropy source a test can turn
//! off, a time counter, and a serial line that keeps what it was told.

use std::cell::RefCell;
use std::sync::Mutex;

thread_local! {
    static ENTROPY_OK: RefCell<bool> = const { RefCell::new(true) };
}

pub static SERIAL: Mutex<Vec<u8>> = Mutex::new(Vec::new());

pub fn entropy_works(ok: bool) {
    ENTROPY_OK.with(|e| *e.borrow_mut() = ok);
}

pub(crate) fn get_entropy64_secure() -> Result<u64, ()> {
    ENTROPY_OK.with(|e| if *e.borrow() { Ok(0x0123_4567_89ab_cdef) } else { Err(()) })
}

pub fn read_time_counter() -> u64 {
    42
}

pub fn print(s: &[u8]) {
    SERIAL.lock().unwrap().extend_from_slice(s);
}

/// Where the kernel keeps the time counter and the serial line.
pub mod arch {
    pub use super::read_time_counter;
}

pub mod sys {
    pub mod serial {
        pub use super::super::print;
    }
}
