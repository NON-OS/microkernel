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
//! The class requests a client's ops become: DTR and RTS, the line
//! coding, a break. A device that does not say it takes one is not sent it.

use nonos_usbnet::Bus;

use crate::acm::{control_lines, send_break, set_line, Line, CAP_BREAK, CAP_LINE};
use crate::device::Dev;
use crate::wire::{E_INVAL, E_IO, E_NODEV};

fn send(d: &Dev, setup: nonos_usbnet::Setup, data: &[u8], cap: u8) -> Result<(), i32> {
    let (Some(mut bus), Some(p)) = (d.bus(), d.port()) else { return Err(E_NODEV) };
    if cap != 0 && p.f.capabilities & cap == 0 {
        return Err(E_INVAL);
    }
    bus.control_out(setup, data).map_err(|_| E_IO)
}

fn comm(d: &Dev) -> u8 {
    d.port().map_or(0, |p| p.f.comm)
}

pub fn lines(d: &Dev, lines: u16) -> Result<(), i32> {
    send(d, control_lines(comm(d), lines), &[], CAP_LINE)
}

pub fn line(d: &Dev, body: &[u8]) -> Result<(), i32> {
    let l = Line::from_bytes(body).filter(|_| body.len() == 7).ok_or(E_INVAL)?;
    send(d, set_line(comm(d)), &l.bytes(), CAP_LINE)
}

pub fn brk(d: &Dev, ms: u16) -> Result<(), i32> {
    send(d, send_break(comm(d), ms), &[], CAP_BREAK)
}
