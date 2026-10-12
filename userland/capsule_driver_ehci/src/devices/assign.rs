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
//! Giving a device in its default state an address: address 0's QH set
//! for how the device is reached (link.rs), its first eight descriptor
//! bytes read there, SET_ADDRESS to the slot's number, then the slot's
//! control QH set for the address, the speed and EP0's packet size.

use super::address::AddrError;
use super::table::Table;
use crate::env::{hold, Clock, Mem, Regs};
use crate::facts::bounds::SET_ADDRESS_MS;
use crate::sched::layout::{ctrl_ring, ADDR0_RING, CTRL_BUF};
use crate::sched::link::Link;
use crate::sched::qh::chars;
use crate::sched::take::take_back;
use crate::xfer::control::control;

/// EP0's packet size, from bMaxPacketSize0: 8 at low speed, 8 to 64 at
/// full speed, 64 at high speed (USB 2.0 section 9.6.1).
pub fn ep0_size(link: Link, b: u8) -> u16 {
    match (link.speed, b) {
        (2, _) => 8,
        (3, _) => 64,
        (_, 16 | 32 | 64) => b as u16,
        _ => 8,
    }
}

/// Address the device answering at address 0 on root port `port` as
/// `slot`, reached by `link`; EP0's packet size.
pub fn assign<R: Regs, C: Clock, M: Mem>(
    (m, r, c): (&M, &mut R, &C),
    t: &mut Table,
    (slot, port): (u8, u8),
    link: Link,
) -> Result<u16, AddrError> {
    let op = t.op;
    let set = |r: &mut R, ring: usize, addr: u8, mps: u16| {
        let ch = link.chars(chars(addr, 0, mps, true), true);
        take_back((m, r, c), op, ring, false, Some((ch, link.caps(false))));
    };
    set(r, ADDR0_RING, 0, if link.high() { 64 } else { 8 });
    let get = [0x80, 6, 0, 1, 0, 0, 8, 0];
    control((m, r, c), op, ADDR0_RING, get).map_err(AddrError::Control)?;
    let mut d = [0u8; 8];
    m.read_bytes(CTRL_BUF, &mut d);
    let mps0 = ep0_size(link, d[7]);
    control((m, r, c), op, ADDR0_RING, [0, 5, slot, 0, 0, 0, 0, 0]).map_err(AddrError::Control)?;
    hold(c, SET_ADDRESS_MS);
    set(r, ctrl_ring(slot), slot, mps0);
    if let Some(dev) = t.dev(slot) {
        (dev.port, dev.link) = (port, link);
    }
    Ok(mps0)
}
