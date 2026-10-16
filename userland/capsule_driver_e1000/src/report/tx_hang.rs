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

//! The line a transmit hang leaves: that the part was reset, or the step
//! that stopped the reset. A failed reset is tried again once the watchdog
//! falls due again.

use super::line::Line;

pub fn tx_hang(outcome: Result<(), &'static str>) {
    let mut line = Line::new(b"driver.e1000: tx hang, reset: ");
    line.push(match outcome {
        Ok(()) => b"done",
        Err(e) => e.as_bytes(),
    });
    line.say();
}
