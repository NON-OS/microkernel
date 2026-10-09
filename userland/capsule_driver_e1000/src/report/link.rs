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

//! The line for a link change, logged only when STATUS.LU differs from the
//! last state logged, so a stack polling the link does not flood the log.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::constants::status::{STATUS_FD, STATUS_LU, STATUS_SPEED_MASK, STATUS_SPEED_SHIFT};

use super::line::Line;

/// Last link state logged: 0 down, 1 up, 2 not yet logged.
static LINK_SEEN: AtomicU8 = AtomicU8::new(2);

pub fn link(status: u32) {
    let up = status & STATUS_LU != 0;
    if LINK_SEEN.swap(up as u8, Ordering::Relaxed) == up as u8 {
        return;
    }
    if !up {
        return Line::new(b"driver.e1000: link down").say();
    }
    // 8254x manual, STATUS bits 7:6: 00 is 10, 01 is 100, 1x is 1000 Mb/s.
    let mut line = Line::new(match (status & STATUS_SPEED_MASK) >> STATUS_SPEED_SHIFT {
        0 => b"driver.e1000: link up 10 Mb/s",
        1 => b"driver.e1000: link up 100 Mb/s",
        _ => b"driver.e1000: link up 1000 Mb/s",
    });
    line.push(if status & STATUS_FD != 0 { b" full duplex" } else { b" half duplex" });
    line.say();
}
