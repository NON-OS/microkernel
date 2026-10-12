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

//! Addressing a high-speed device: its port reset again (it must stay
//! enabled), its first eight descriptor bytes read at address 0 on address
//! 0's QH, SET_ADDRESS to the slot's number, then the slot's control QH
//! set for the address and EP0's packet size.

use super::table::Table;
use crate::env::{hold, Clock, Mem, Regs};
use crate::facts::bounds::SET_ADDRESS_MS;
use crate::hc::port::{hand, offset, reset};
use crate::sched::layout::{ctrl_ring, ADDR0_RING, CTRL_BUF};
use crate::sched::qh::chars;
use crate::sched::take::take_back;
use crate::xfer::control::control;
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
    let (op, off) = (t.op, offset(t.op, port));
    if !t.high[port as usize - 1] || reset(r, c, off) != Ok(true) {
        hand(r, off);
        t.high[port as usize - 1] = false;
        return Err(AddrError::NotHere);
    }
    let get = [0x80, 6, 0, 1, 0, 0, 8, 0];
    control((m, r, c), op, ADDR0_RING, get).map_err(AddrError::Control)?;
    let mut d = [0u8; 8];
    m.read_bytes(CTRL_BUF, &mut d);
    let mps0 = if d[7] == 64 { 64 } else { d[7].max(8) as u16 };
    control((m, r, c), op, ADDR0_RING, [0, 5, slot, 0, 0, 0, 0, 0]).map_err(AddrError::Control)?;
    hold(c, SET_ADDRESS_MS);
    take_back((m, r, c), op, ctrl_ring(slot), false, Some(chars(slot, 0, mps0, true)));
    if let Some(dev) = t.dev(slot) {
        dev.port = port;
    }
    Ok(mps0)
}
