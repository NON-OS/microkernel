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

//! The line a finished bring-up leaves: CTRL and STATUS as the part holds
//! them once both rings are enabled.

use super::line::Line;

pub fn up(ctrl: u32, status: u32) {
    let mut line = Line::new(b"driver.e1000: up, ctrl=");
    line.hex(ctrl);
    line.push(b" status=");
    line.hex(status);
    line.say();
}
