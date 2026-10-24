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

//! The clock: each reading, in any unit, is one tick past the last.

use std::cell::Cell;

thread_local! {
    static TICKS: Cell<u64> = const { Cell::new(1_000) };
}

pub fn current_ticks() -> u64 {
    TICKS.set(TICKS.get() + 1);
    TICKS.get()
}
pub fn timestamp_millis() -> u64 {
    current_ticks()
}
pub fn timestamp_secs() -> u64 {
    current_ticks()
}
pub fn get_kernel_time_ns() -> u64 {
    current_ticks()
}
