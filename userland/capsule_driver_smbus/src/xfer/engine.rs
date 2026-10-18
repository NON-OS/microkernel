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

//! One transfer from start to finish: the host taken while idle, loaded
//! from the plan, started, watched to its end, read out, and given back.
//!
//! HST_STS.INUSE is the semaphore the host shares with firmware and ACPI:
//! reading the status sets it, writing it back clears it. Every transfer
//! here reads the status, so every transfer, however it ends, gives the
//! semaphore back.

use super::buffer::exchange;
use super::plan::{plan, Data, Plan};
use super::request::{Xfer, BLOCK_MAX};
use super::take::take;
use super::watch::watch;
use crate::ports::Ports;
use crate::regs::{HSTADD, HSTCMD, HSTCNT, HSTDAT0, HSTDAT1, HSTSTS, STS_FLAGS, STS_INUSE};

/// Runs `x` and returns how many bytes of `out` a read filled.
pub fn transfer<P: Ports>(p: &mut P, x: &Xfer, out: &mut [u8; BLOCK_MAX]) -> Result<usize, i32> {
    let plan = plan(x);
    let moved = take(p).and_then(|()| carry(p, &plan, x, out));
    let given = p.w8(HSTSTS, STS_INUSE | STS_FLAGS);
    let n = moved?;
    given.map(|()| n)
}

fn carry<P: Ports>(
    p: &mut P,
    plan: &Plan,
    x: &Xfer,
    out: &mut [u8; BLOCK_MAX],
) -> Result<usize, i32> {
    p.w8(HSTADD, plan.address)?;
    if let Some(c) = plan.command {
        p.w8(HSTCMD, c)?;
    }
    if plan.data == Data::Buffer {
        return exchange(p, plan, x, out);
    }
    let regs = data_regs(plan.data);
    if !plan.read {
        regs.iter().zip(&x.data).try_for_each(|(&r, &b)| p.w8(r, b))?;
    }
    p.w8(HSTCNT, plan.start)?;
    watch(p)?;
    if !plan.read {
        return Ok(0);
    }
    regs.iter().zip(out.iter_mut()).try_for_each(|(&r, b)| p.r8(r).map(|v| *b = v))?;
    Ok(regs.len())
}

fn data_regs(d: Data) -> &'static [u16] {
    match d {
        Data::Low => &[HSTDAT0],
        Data::Pair => &[HSTDAT0, HSTDAT1],
        Data::None | Data::Buffer => &[],
    }
}
