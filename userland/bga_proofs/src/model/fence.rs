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

//! The fence the driver calls after a copy, counted per test thread so a
//! test can see the copy was ordered before the answer.

use std::cell::Cell;

thread_local! {
    static FENCES: Cell<usize> = const { Cell::new(0) };
}

pub fn sfence() {
    FENCES.with(|f| f.set(f.get() + 1));
}

pub fn fences() -> usize {
    FENCES.with(|f| f.get())
}
