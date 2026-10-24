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

//! Addressing a device on a root port: the port reset and enabled, then
//! the device given its address (assign.rs).

use super::assign::assign;
use super::table::Table;
use crate::env::{Clock, Io, Mem};
use crate::hc::port::{reset, valid};
use crate::xfer::control::CtrlError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AddrError {
    /// Another slot holds the port, or the slot is not enabled.
    Busy,
    BadRequest,
    Port(&'static str),
    Control(CtrlError),
}

/// The device's speed (low or full) and EP0's packet size.
pub fn address<I: Io, C: Clock, M: Mem>(
    io: &mut I,
    c: &C,
    m: &M,
    t: &mut Table,
    (slot, port): (u8, u8),
) -> Result<(bool, u16), AddrError> {
    if !valid(port) || t.dev(slot).is_none_or(|d| d.port != 0) {
        return Err(AddrError::BadRequest);
    }
    if t.holder(port).is_some() {
        return Err(AddrError::Busy);
    }
    let low = reset(io, c, port).map_err(AddrError::Port)?;
    assign(m, c, t, (slot, port), low)
}
