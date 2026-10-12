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

//! A bulk endpoint on its slot's own bulk QH in the async ring: one qTD a
//! transfer, the controller cutting it into packets of the QH's size; the
//! toggle is the QH's, kept across transfers. A short IN packet ends the
//! qTD. bulk_run.rs runs one transfer and resets the QH.

use super::failed::{failed, CtrlError};
use crate::env::{Clock, Mem, Regs};
use crate::facts::qtd::{PID_IN, PID_OUT};
use crate::sched::layout::qh;
use crate::sched::qh::chars;
use crate::sched::qtd::{moved, write, Qtd};
use crate::sched::queue::{start, Ran};
use crate::sched::take::{clear_halt, take_back};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BulkEp {
    pub addr: u8,
    pub ep: u8,
    pub mps: u16,
    /// The QH's ring index.
    pub ring: usize,
}

/// The QH set for the endpoint, DATA0 next: taken from the ring for it.
pub fn configure<M: Mem, R: Regs, C: Clock>(mrc: (&M, &mut R, &C), op: u32, ep: &BulkEp) {
    let ch = chars(ep.addr, ep.ep, ep.mps, false);
    take_back(mrc, op, ep.ring, false, Some(ch));
}

/// One qTD of `len` bytes at `buf`, written at `qtd` and handed over.
pub fn arm<M: Mem>(m: &M, ep: &BulkEp, input: bool, (qtd, buf, len): (usize, usize, usize)) {
    let pid = if input { PID_IN } else { PID_OUT };
    write(m, qtd, Qtd { pid, toggle: false, buf, len }, None, None);
    start(m, qh(ep.ring), qtd);
}

/// How the run ended, as the bytes moved. A halt is emptied, the toggle
/// kept: the class driver resets the pipe after the STALL it is told of.
pub fn ended<M: Mem>(
    m: &M,
    ep: &BulkEp,
    ran: Ran,
    qtd: usize,
    len: usize,
) -> Result<usize, CtrlError> {
    match ran {
        Ran::Done => Ok(moved(m, qtd, len)),
        Ran::Halted(status) => {
            clear_halt(m, qh(ep.ring), true);
            Err(failed(status))
        }
        Ran::TimedOut => Err(CtrlError::TimedOut),
    }
}
