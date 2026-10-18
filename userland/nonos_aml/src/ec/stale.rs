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

//! Bytes the EC left in its output buffer, read away before each command.
//! Sixteen is our bound, from no source: an EC that offers more is lying.

use super::protocol::status;
use super::regs::{EcError, EcPorts, OBF};
use crate::events::EventIo;

const STALE: u32 = 16;

/// A byte left in the output buffer answers no command of ours: read it
/// away first, so an EC answering out of order cannot shift every answer.
pub(super) fn flush(p: &EcPorts, io: &mut impl EventIo) -> Result<(), EcError> {
    for _ in 0..STALE {
        if status(p, io)? & OBF == 0 {
            return Ok(());
        }
        io.read(p.data, 1).ok_or(EcError::Io)?;
    }
    Err(EcError::Timeout)
}
