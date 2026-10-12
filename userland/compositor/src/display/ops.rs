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

//! The common ops the compositor sends (section 12): CAPS, SURFACE, STATE,
//! FLUSH and RETIRE, each to one output (0 is the first). A driver answering
//! CAPS with ENOSYS speaks only the legacy ops, and the caller drives it
//! that way.

use nonos_gfx_wire::body::flush::MAX_RECTS;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{ops, Caps, Errno, Flush, Rect, State, Surface};

use super::call::{call, PRESENT_MS, SETUP_MS};

pub fn caps(port: u32) -> Result<Caps, Errno> {
    let mut b = [0u8; 32];
    let n = call((port, 0), ops::CAPS, &[], &mut b, SETUP_MS)?;
    Caps::decode(&b[..n])
}

/// Output `output`'s current primary; asking makes this compositor its owner.
pub fn surface(port: u32, output: u16) -> Result<Surface, Errno> {
    let mut b = [0u8; 32];
    let n = call((port, output), ops::SURFACE, &[], &mut b, SETUP_MS)?;
    Surface::decode(&b[..n])
}

pub fn state(port: u32, output: u16) -> Result<State, Errno> {
    let mut b = [0u8; 32];
    let n = call((port, output), ops::STATE, &[], &mut b, SETUP_MS)?;
    State::decode(&b[..n])
}

/// Make `rects` of primary `serial` on `output` visible; at most 15 a call.
pub fn flush(port: u32, output: u16, serial: u32, rects: &[Rect]) -> Result<(), Errno> {
    if rects.is_empty() || rects.len() > MAX_RECTS {
        return Err(EINVAL);
    }
    let mut f = Flush { serial, len: rects.len(), ..Flush::default() };
    f.rects[..rects.len()].copy_from_slice(rects);
    let mut p = [0u8; 8 + 16 * MAX_RECTS];
    let n = f.encode(&mut p)?;
    call((port, output), ops::FLUSH, &p[..n], &mut [], PRESENT_MS).map(|_| ())
}

/// Let the driver free a primary the compositor no longer shows; a serial
/// names one primary across every output, so output 0 carries it.
pub fn retire(port: u32, serial: u32) -> Result<(), Errno> {
    call((port, 0), ops::RETIRE, &serial.to_le_bytes(), &mut [], SETUP_MS).map(|_| ())
}
