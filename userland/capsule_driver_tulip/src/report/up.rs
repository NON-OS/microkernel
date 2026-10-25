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

//! The line a finished bring-up leaves: which part, and CSR5 with both
//! processes running.

use super::line::Line;

pub fn up(device: u16, csr5: u32) {
    let mut line = Line::new(b"driver.tulip: up, 1011:");
    line.hex16(device);
    line.push(b" csr5=");
    line.hex16((csr5 >> 16) as u16);
    line.hex16(csr5 as u16);
    line.say();
}
