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

//! One look at the interrupt status, judged. The driver signals no
//! interrupts; it polls these status bits, which latch because their status
//! enables are set.

use super::regs::INT_ERROR;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seen {
    /// Neither the awaited status nor an error yet.
    Pending,
    /// The awaited status is set and no error is.
    Done,
    /// An error is posted; the value is the Error Interrupt Status.
    Failed(u16),
}

/// Judge the 32-bit read of Normal (15:0) and Error (31:16) Interrupt
/// Status while waiting for `want`. An error wins over completion: a
/// command can post Command Complete and still have failed its data phase.
pub const fn judge(status: u32, want: u16) -> Seen {
    let normal = status as u16;
    let error = (status >> 16) as u16;
    if error != 0 || normal & INT_ERROR != 0 {
        return Seen::Failed(error);
    }
    if normal & want == want {
        return Seen::Done;
    }
    Seen::Pending
}
