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

//! What a halted qTD's status said, and the errors a transfer answers. A
//! qTD halted with no error bit beside the halt was refused: a STALL.

use crate::facts::qtd::TOK_ERRORS;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CtrlError {
    Stalled,
    /// A babble, buffer or transaction error: the status bits.
    Bus(u32),
    TimedOut,
    /// Longer than the data buffer.
    TooLong,
}

pub fn failed(status: u32) -> CtrlError {
    match status & TOK_ERRORS {
        0 => CtrlError::Stalled,
        bits => CtrlError::Bus(bits),
    }
}
