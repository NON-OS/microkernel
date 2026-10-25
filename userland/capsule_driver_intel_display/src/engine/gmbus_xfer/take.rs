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

//! Taking the bytes of a GMBUS read as the controller readies them: one
//! word for each four bytes asked, never more whatever the controller
//! still offers, every wait drawn from the transaction's one deadline.

use super::super::access::{Expiry, Mmio};
use super::super::error::Error;
use super::super::gmbus_regs::{GMBUS2, GMBUS3, HW_RDY, HW_WAIT_PHASE, NAK};

/// GMBUS2 once it shows any of `bits` or a NAK; Timeout when `until` ends
/// first. The deadline is asked even of a status that is ready, so a sink
/// that answers every poll at once but holds the clock between them is
/// cut off all the same.
pub fn wait_any(m: &mut dyn Mmio, bits: u32, until: &dyn Expiry) -> Result<u32, Error> {
    loop {
        let st = m.read(GMBUS2);
        if until.expired() {
            return Err(Error::Timeout);
        }
        if st & (bits | NAK) != 0 {
            return Ok(st);
        }
    }
}

pub fn take(m: &mut dyn Mmio, until: &dyn Expiry, out: &mut [u8]) -> Result<(), Error> {
    for chunk in out.chunks_mut(4) {
        if wait_any(m, HW_RDY, until)? & NAK != 0 {
            return Err(Error::Layout);
        }
        let w = m.read(GMBUS3).to_le_bytes();
        chunk.copy_from_slice(&w[..chunk.len()]);
    }
    if wait_any(m, HW_WAIT_PHASE, until)? & NAK != 0 {
        return Err(Error::Layout);
    }
    Ok(())
}
