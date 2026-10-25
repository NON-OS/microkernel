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

//! The tests' clock for the leave schedule (rule.rs).

use std::cell::Cell;

/// A clock that moves a millisecond a read and fails the test if a run
/// outlives every budget (a missing bound otherwise hangs).
pub(super) fn clock(t: &Cell<u64>) -> impl FnMut() -> Option<u64> + '_ {
    move || {
        t.set(t.get() + 1);
        assert!(t.get() < 100_000, "the leave phase outlived every budget");
        Some(t.get())
    }
}
