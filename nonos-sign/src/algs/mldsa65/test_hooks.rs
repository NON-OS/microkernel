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

//! The tests' switch for a failing system randomness source.

use super::ffi;

/// Draws fail on this thread while the returned value lives, and work
/// again once it is dropped, even if the test panics first.
pub(super) struct Forced;

impl Forced {
    pub(super) fn on() -> Self {
        // SAFETY: eK@nonos.systems - a thread-local int in the C.
        unsafe { ffi::NONOS_mldsa65_force_random_failure(1) };
        Forced
    }
}

impl Drop for Forced {
    fn drop(&mut self) {
        // SAFETY: eK@nonos.systems - as above.
        unsafe { ffi::NONOS_mldsa65_force_random_failure(0) };
    }
}
