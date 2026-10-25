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

//! The EC's events drained (ACPI 6.5, 12.6): while SCI_EVT is set, QR_EC
//! for the next one. At most `MAX_QUERIES` a call, so an EC that keeps
//! answering cannot hold the caller; and past the storm bound in a window
//! (`events::Storms`, one key for the EC) it is reported as flooding.

use super::query::{pending, query};
use super::regs::{EcError, EcPorts};
use crate::events::{EventIo, Storms};
use alloc::vec::Vec;

/// More events at once than any EC queues.
pub const MAX_QUERIES: usize = 32;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Drained {
    /// The query values, in the order the EC gave them.
    pub queries: Vec<u8>,
    pub flooded: bool,
}

pub fn drain(p: &EcPorts, io: &mut impl EventIo, storms: &mut Storms) -> Result<Drained, EcError> {
    let (now, mut out) = (io.now_ms(), Drained::default());
    while out.queries.len() < MAX_QUERIES && pending(p, io)? {
        if storms.hit(0, now) {
            out.flooded = true;
            break;
        }
        match query(p, io)? {
            Some(q) => out.queries.push(q),
            None => break,
        }
    }
    Ok(out)
}
