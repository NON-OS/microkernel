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

//! The EC's events: SCI_EVT in its status, and QR_EC to learn which.

use super::protocol::{get, put, status};
use super::regs::{EcError, EcPorts, QR_EC, SCI_EVT};
use super::stale::flush;
use crate::events::EventIo;

/// Whether the EC has an event to report (SCI_EVT).
pub fn pending(p: &EcPorts, io: &mut impl EventIo) -> Result<bool, EcError> {
    Ok(status(p, io)? & SCI_EVT != 0)
}

/// QR_EC: the next event's number, None when the EC has none (12.3.5:
/// zero means no outstanding event).
pub fn query(p: &EcPorts, io: &mut impl EventIo) -> Result<Option<u8>, EcError> {
    flush(p, io)?;
    put(p, io, p.command, QR_EC)?;
    let q = get(p, io)?;
    Ok((q != 0).then_some(q))
}
