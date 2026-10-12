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

//! The uptime clock, set by the test, and a wait that counts each question
//! as the millisecond the driver sleeps between them.

use std::cell::Cell;

thread_local!(static NOW: Cell<u64> = const { Cell::new(0) });

pub fn set(ms: u64) {
    NOW.with(|n| n.set(ms));
}

pub fn now_ms() -> u64 {
    NOW.with(Cell::get)
}

pub fn expired_after(ms: u64) -> impl FnMut() -> bool {
    let mut spent = 0u64;
    move || {
        spent += 1;
        spent >= ms
    }
}
