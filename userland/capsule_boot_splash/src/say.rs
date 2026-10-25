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

//! The splash's two lines on the kernel's log (with the Debug capability):
//! up, and stepped aside with why. A failed step before it is up exits with
//! that step's code instead, which the kernel logs without Debug.

use alloc::format;

use nonos_libc::mk_debug;

pub(crate) fn line(text: &str) {
    let l = format!("[SPLASH] {}\n", text);
    let _ = mk_debug(l.as_ptr(), l.len());
}

/// The splash stepping aside at `now`, and why.
pub(crate) fn stepped_aside(now: i64, desktop_up_at: i64) {
    line(&format!("stepped aside at {} ms: {}", now, crate::dwell::why(desktop_up_at)));
}
