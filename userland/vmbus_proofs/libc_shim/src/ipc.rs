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

//! The console, and a virtual clock: a sleep moves it on, so a wait that
//! runs out costs no real time.

use std::cell::{Cell, RefCell};

thread_local! {
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static NOW: Cell<u64> = const { Cell::new(0) };
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    // SAFETY: the caller hands `len` readable bytes, as the real call requires.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    LOG.with(|l| l.borrow_mut().push(String::from_utf8_lossy(bytes).into_owned()));
    len as i64
}

/// Every console line on this thread.
pub fn logged() -> Vec<String> {
    LOG.with(|l| l.borrow().clone())
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    NOW.with(|n| n.set(n.get() + ms));
    0
}

pub fn mk_yield() -> i64 {
    0
}

/// The virtual clock, in milliseconds.
pub fn now_ms() -> u64 {
    NOW.with(Cell::get)
}

pub struct Deadline {
    end: u64,
}

impl Deadline {
    pub fn after_ms(ms: u64) -> Self {
        Self { end: now_ms() + ms }
    }
    pub fn expired(&self) -> bool {
        now_ms() >= self.end
    }
}
