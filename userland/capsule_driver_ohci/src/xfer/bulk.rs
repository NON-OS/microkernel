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

//! A bulk endpoint: one TD per transfer on the endpoint's own ED, which
//! the controller cuts into packets of the ED's size; the toggle is the
//! ED's carry, kept across transfers. A short IN packet ends the TD.

use super::failed::{failed, CtrlError};
use crate::env::{Mem, Regs};
use crate::facts::regs::CS_BLF;
use crate::facts::td::{DP_IN, DP_OUT};
use crate::sched::ed::{control as ed_word, Target};
use crate::sched::layout::{bulk_ed, bulk_td};
use crate::sched::queue::{start, Ran};
use crate::sched::take::clear_halt;
use crate::sched::td::{moved, write, Td};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BulkEp {
    pub t: Target,
    pub mps: u16,
    /// Which of the BULK_EDS EDs and TD pairs is this endpoint's.
    pub index: usize,
    /// The place the queue rests on.
    pub at: u8,
}

/// The ED set for the endpoint; its queue is empty, so it runs nothing yet.
pub fn configure<M: Mem>(m: &M, ep: &BulkEp) {
    m.wr(bulk_ed(ep.index), ed_word(ep.t, ep.mps, false));
}

/// One TD of `len` bytes at `buf` on the resting place, handed over.
pub fn arm<M: Mem, R: Regs>(m: &M, r: &mut R, ep: &BulkEp, input: bool, buf: usize, len: usize) {
    let dp = if input { DP_IN } else { DP_OUT };
    let td = Td { dp, toggle: None, round: input, buf, len };
    let (here, tail) = (bulk_td(ep.index, ep.at), bulk_td(ep.index, 1 - ep.at));
    write(m, here, td, tail);
    start(m, r, bulk_ed(ep.index), tail, CS_BLF);
}

/// How the run ended, as the bytes moved, the queue now on the other place.
/// A halt is cleared, carry kept: the class driver resets the pipe.
pub fn ended<M: Mem>(
    m: &M,
    ep: &mut BulkEp,
    ran: Ran,
    buf: usize,
    len: usize,
) -> Result<usize, CtrlError> {
    let here = bulk_td(ep.index, ep.at);
    ep.at = 1 - ep.at;
    match ran {
        Ran::Done => Ok(moved(m, here, buf, len)),
        Ran::Halted => {
            clear_halt(m, bulk_ed(ep.index), true);
            Err(failed(m, here, 1))
        }
        Ran::TimedOut => Err(CtrlError::TimedOut),
    }
}
