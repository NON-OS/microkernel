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

//! Before and after a transaction, as Linux's i801_check_pre and
//! i801_check_post have it.

use crate::errno::{E_AGAIN, E_BUSY, E_IO, E_NXIO};
use crate::ports::Ports;
use crate::regs::STS_INUSE;
use crate::regs::{HSTSTS, STS_BUS_ERR, STS_DEV_ERR, STS_FAILED, STS_FLAGS, STS_HOST_BUSY};

/// A busy host is refused. What a transaction before this one left in the
/// status is cleared first, and a status that will not clear is busy too.
pub fn check_pre<P: Ports>(p: &mut P) -> Result<(), i32> {
    let st = p.r8(HSTSTS)?;
    if st & STS_HOST_BUSY != 0 {
        return Err(E_BUSY);
    }
    let left = st & STS_FLAGS;
    if left != 0 {
        p.w8(HSTSTS, left)?;
        if p.r8(HSTSTS)? & STS_FLAGS != 0 {
            return Err(E_BUSY);
        }
    }
    Ok(())
}

/// The finished status as an answer. Linux tests the bits in this order and
/// keeps the last that is set: a lost arbitration over a missing device over
/// a failed transaction.
pub fn verdict(st: u8) -> Result<(), i32> {
    if st & STS_BUS_ERR != 0 {
        Err(E_AGAIN)
    } else if st & STS_DEV_ERR != 0 {
        Err(E_NXIO)
    } else if st & STS_FAILED != 0 {
        Err(E_IO)
    } else {
        Ok(())
    }
}

/// Clears what the transaction left and gives the host back to firmware
/// and ACPI, whose semaphore reading the status took.
pub fn release<P: Ports>(p: &mut P) -> Result<(), i32> {
    p.w8(HSTSTS, STS_INUSE | STS_FLAGS)
}
