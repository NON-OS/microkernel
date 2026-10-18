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

//! The EC's handshake (ACPI 6.5, 12.3): before each write the input buffer
//! must be empty; before each read the output buffer full. Every wait is
//! bounded: a few looks back to back, since an EC usually answers within
//! microseconds, then a look a millisecond, up to a second in all.

use super::regs::{EcError, EcPorts, IBF, OBF, RD_EC, WR_EC};
use super::stale::flush;
use crate::events::EventIo;

const SPINS: u32 = 64;
const SLEEPS_MS: u32 = 1000;

pub(super) fn status(p: &EcPorts, io: &mut impl EventIo) -> Result<u16, EcError> {
    io.read(p.command, 1).ok_or(EcError::Io)
}

fn wait(p: &EcPorts, io: &mut impl EventIo, done: impl Fn(u16) -> bool) -> Result<(), EcError> {
    for i in 0..SPINS + SLEEPS_MS {
        if done(status(p, io)?) {
            return Ok(());
        }
        if i >= SPINS {
            io.sleep(1);
        }
    }
    Err(EcError::Timeout)
}

pub(super) fn put(p: &EcPorts, io: &mut impl EventIo, port: u16, v: u16) -> Result<(), EcError> {
    wait(p, io, |s| s & IBF == 0)?;
    io.write(port, 1, v).then_some(()).ok_or(EcError::Io)
}

pub(super) fn get(p: &EcPorts, io: &mut impl EventIo) -> Result<u8, EcError> {
    wait(p, io, |s| s & OBF != 0)?;
    io.read(p.data, 1).map(|v| v as u8).ok_or(EcError::Io)
}

/// RD_EC: one byte of the EC's address space.
pub fn read(p: &EcPorts, io: &mut impl EventIo, addr: u8) -> Result<u8, EcError> {
    flush(p, io)?;
    put(p, io, p.command, RD_EC)?;
    put(p, io, p.data, addr as u16)?;
    get(p, io)
}

/// WR_EC: one byte written, and the EC seen to take it.
pub fn write(p: &EcPorts, io: &mut impl EventIo, addr: u8, v: u8) -> Result<(), EcError> {
    flush(p, io)?;
    put(p, io, p.command, WR_EC)?;
    put(p, io, p.data, addr as u16)?;
    put(p, io, p.data, v as u16)?;
    wait(p, io, |s| s & IBF == 0)
}
