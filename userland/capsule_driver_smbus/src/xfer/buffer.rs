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

//! Block transfers through the host's 32-byte buffer (AUX_CTL.E32B, ICH4
//! and later). With the buffer on, Host Block Data is a window onto it
//! whose index a read of HST_CNT puts back at the start; HST_D0 holds the
//! count, written before a block write and returned by a block read.

use super::plan::Plan;
use super::request::{Xfer, BLOCK_MAX};
use super::watch::watch;
use crate::errno::{E_NOTSUP, E_PROTO};
use crate::ports::Ports;
use crate::regs::{AUXCTL, AUX_CRC, AUX_E32B, BLKDAT, HSTCNT, HSTDAT0};

/// The buffer is on only for the transfer and off after it, however the
/// transfer ends. The appended CRC (PEC) is off throughout, whatever
/// firmware left: this driver asks for no PEC, so none is sent or checked.
/// A host where E32B does not stay set has no buffer.
pub fn exchange<P: Ports>(
    p: &mut P,
    plan: &Plan,
    x: &Xfer,
    out: &mut [u8; BLOCK_MAX],
) -> Result<usize, i32> {
    let aux = p.r8(AUXCTL)?;
    p.w8(AUXCTL, (aux | AUX_E32B) & !AUX_CRC)?;
    if p.r8(AUXCTL)? & AUX_E32B == 0 {
        return Err(E_NOTSUP);
    }
    let moved = through(p, plan, x, out);
    let aux = p.r8(AUXCTL)?;
    p.w8(AUXCTL, aux & !(AUX_E32B | AUX_CRC))?;
    moved
}

fn through<P: Ports>(
    p: &mut P,
    plan: &Plan,
    x: &Xfer,
    out: &mut [u8; BLOCK_MAX],
) -> Result<usize, i32> {
    p.r8(HSTCNT)?;
    if !plan.read {
        p.w8(HSTDAT0, x.len)?;
        p.r8(HSTCNT)?;
        x.data[..x.len as usize].iter().try_for_each(|&b| p.w8(BLKDAT, b))?;
    }
    p.w8(HSTCNT, plan.start)?;
    watch(p)?;
    if !plan.read {
        return Ok(0);
    }
    let count = p.r8(HSTDAT0)? as usize;
    if !(1..=BLOCK_MAX).contains(&count) {
        return Err(E_PROTO);
    }
    p.r8(HSTCNT)?;
    out[..count].iter_mut().try_for_each(|b| p.r8(BLKDAT).map(|v| *b = v))?;
    Ok(count)
}
