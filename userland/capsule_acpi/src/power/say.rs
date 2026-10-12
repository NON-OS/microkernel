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

//! One line when what the kernel is told changes.

use super::report::{AC_KNOWN, AC_ONLINE, BATTERY_PRESENT, UNKNOWN_PERCENT};
use crate::sys::log::Line;

pub(super) fn say(w: u64) {
    let l = Line::new(b"power battery=").str(if w & BATTERY_PRESENT != 0 { b"yes" } else { b"no" });
    let l = match w & 0xFF {
        UNKNOWN_PERCENT => l.str(b" charge=unknown"),
        pc => l.str(b" charge=").dec(pc),
    };
    let ac: &[u8] = match (w & AC_KNOWN != 0, w & AC_ONLINE != 0) {
        (true, true) => b"online",
        (true, false) => b"offline",
        _ => b"unknown",
    };
    l.str(b" ac=").str(ac).end();
}
