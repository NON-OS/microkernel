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
//! Addressing a high-speed device on a root port: its port reset again
//! (it must stay enabled), then given its address (assign.rs).

use super::assign::assign;
use super::table::Table;
use crate::env::{Clock, Mem, Regs};
use crate::hc::port::{hand, offset, reset};
use crate::sched::link::Link;
use crate::xfer::failed::CtrlError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AddrError {
    /// Another slot holds the port, or the slot is not enabled.
    Busy,
    BadRequest,
    /// Not a high-speed device of this host's (or no longer).
    NotHere,
    Control(CtrlError),
}

/// EP0's packet size.
pub fn address<R: Regs, C: Clock, M: Mem>(
    (m, r, c): (&M, &mut R, &C),
    t: &mut Table,
    (slot, port): (u8, u8),
) -> Result<u16, AddrError> {
    let unaddressed = t.dev(slot).is_some_and(|d| d.port == 0);
    if !(1..=t.ports).contains(&port) || !unaddressed {
        return Err(AddrError::BadRequest);
    }
    if t.holder(port).is_some() {
        return Err(AddrError::Busy);
    }
    let off = offset(t.op, port);
    if !t.high[port as usize - 1] || reset(r, c, off) != Ok(true) {
        hand(r, off);
        t.high[port as usize - 1] = false;
        return Err(AddrError::NotHere);
    }
    assign((m, r, c), t, (slot, port), Link::HIGH)
}
