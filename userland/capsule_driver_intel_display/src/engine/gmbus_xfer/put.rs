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

//! The bytes of a GMBUS write as the controller takes them: the first four
//! in GMBUS3 before the command goes, each next four once it shows ready,
//! then the wait phase after the last; a NAK on the way ends it. Left in
//! that phase, the next command follows with a repeated start.

use super::super::access::{Expiry, Mmio};
use super::super::error::Error;
use super::super::gmbus_regs::{write_cmd, GMBUS1, GMBUS3, HW_RDY, HW_WAIT_PHASE, NAK};
use super::take::wait_any;

/// Write `bytes` (at most 0x1FF) to 7-bit `addr` on the selected pins.
pub fn put(m: &mut dyn Mmio, until: &dyn Expiry, addr: u32, bytes: &[u8]) -> Result<(), Error> {
    let mut words = bytes.chunks(4).map(word);
    m.write(GMBUS3, words.next().unwrap_or(0));
    m.write(GMBUS1, write_cmd(addr, bytes.len()));
    for w in words {
        acked(wait_any(m, HW_RDY, until)?)?;
        m.write(GMBUS3, w);
    }
    acked(wait_any(m, HW_WAIT_PHASE, until)?)
}

fn word(c: &[u8]) -> u32 {
    let mut w = [0u8; 4];
    w[..c.len()].copy_from_slice(c);
    u32::from_le_bytes(w)
}

fn acked(st: u32) -> Result<(), Error> {
    if st & NAK != 0 {
        return Err(Error::Layout);
    }
    Ok(())
}
