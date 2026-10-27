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
//! qTD. bulk_run.rs sets the QHs, runs one transfer and resets the QH.

use super::failed::{failed, CtrlError};
use crate::env::Mem;
use crate::facts::qtd::{PID_IN, PID_OUT};
use crate::sched::layout::qh;
use crate::sched::link::Link;
use crate::sched::qtd::{moved, write, Qtd};
use crate::sched::queue::{start, Ran};
use crate::sched::take::clear_halt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BulkEp {
    pub addr: u8,
    pub ep: u8,
    pub mps: u16,
    /// The QH's ring index.
    pub ring: usize,
    pub link: Link,
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
