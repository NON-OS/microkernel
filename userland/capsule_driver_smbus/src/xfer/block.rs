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

//! A block transfer through the 32-byte buffer (Linux's
//! i801_block_transaction_by_block).

use super::check::verdict;
use super::request::{Xfer, BLOCK_MAX};
use super::wait::wait_done;
use crate::errno::{E_NOTSUP, E_PROTO};
use crate::ports::Ports;
use crate::regs::{AUXCTL, AUX_CRC, AUX_E32B, BLKDAT, CNT_BLOCK_DATA, CNT_START, HSTCNT, HSTDAT0};

/// The buffer is turned on for the transfer and off after it, whatever the
/// transfer's outcome. A host that will not take it predates ICH4.
pub fn block<P: Ports>(p: &mut P, x: &Xfer, out: &mut [u8; BLOCK_MAX]) -> Result<usize, i32> {
    let aux = p.r8(AUXCTL)?;
    p.w8(AUXCTL, aux | AUX_E32B)?;
    if p.r8(AUXCTL)? & AUX_E32B == 0 {
        return Err(E_NOTSUP);
    }
    let r = buffered(p, x, out);
    let aux = p.r8(AUXCTL)?;
    p.w8(AUXCTL, aux & !(AUX_CRC | AUX_E32B))?;
    r
}

fn buffered<P: Ports>(p: &mut P, x: &Xfer, out: &mut [u8; BLOCK_MAX]) -> Result<usize, i32> {
    // Reading the control register resets the buffer's index.
    p.r8(HSTCNT)?;
    if !x.read {
        p.w8(HSTDAT0, x.len)?;
        p.r8(HSTCNT)?;
        for &b in &x.data[..x.len as usize] {
            p.w8(BLKDAT, b)?;
        }
    }
    p.w8(HSTCNT, CNT_BLOCK_DATA | CNT_START)?;
    verdict(wait_done(p)?)?;
    if !x.read {
        return Ok(0);
    }
    let len = p.r8(HSTDAT0)? as usize;
    if !(1..=BLOCK_MAX).contains(&len) {
        return Err(E_PROTO);
    }
    p.r8(HSTCNT)?;
    for slot in &mut out[..len] {
        *slot = p.r8(BLKDAT)?;
    }
    Ok(len)
}
