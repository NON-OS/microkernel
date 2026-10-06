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

//! The line a link change writes.

use crate::constants::status::{STATUS_FD, STATUS_LU, STATUS_SPEED_MASK, STATUS_SPEED_SHIFT};
use crate::log::Line;

/// `e1000e: link up 1000 full` or `e1000e: link down`. STATUS.SPEED is
/// 00 for 10, 01 for 100 and 1x for 1000 Mb/s (E1000_STATUS_SPEED_*).
pub fn log_change(status: u32) {
    let mut line = Line::new();
    if status & STATUS_LU == 0 {
        line.text("link down").send();
        return;
    }
    let speed = match (status & STATUS_SPEED_MASK) >> STATUS_SPEED_SHIFT {
        0 => "10",
        1 => "100",
        _ => "1000",
    };
    let duplex = if status & STATUS_FD != 0 { " full" } else { " half" };
    line.text("link up ").text(speed).text(duplex).send();
}
