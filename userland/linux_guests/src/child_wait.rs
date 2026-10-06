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

//! Waiting for a forked child, bounded, so a child that never runs is
//! reported instead of hanging the probe.

use crate::sys::{call, NANOSLEEP};

const WAIT4: u64 = 61;
const WNOHANG: u64 = 1;

// The child's exit status, or None if it did not finish within ten seconds.
pub fn wait(child: i64) -> Option<u32> {
    let mut status = 0i32;
    for _ in 0..100 {
        let rc = call(WAIT4, [child as u64, &mut status as *mut i32 as u64, WNOHANG, 0, 0, 0]);
        if rc == child {
            return Some(((status >> 8) & 0xff) as u32);
        }
        let tenth = [0u64, 100_000_000];
        let _ = call(NANOSLEEP, [tenth.as_ptr() as u64, 0, 0, 0, 0, 0]);
    }
    None
}
