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

//! An interrupt IN endpoint: one TD always armed on its own queue, which
//! the controller retries every frame while the device NAKs. A poll looks
//! at the TD once: still active is "nothing yet"; done hands back the report
//! and arms the next with the toggle flipped.

use super::chain::{write, Step};
use crate::env::Mem;
use crate::facts::td::{PID_IN, STATUS_STALLED};
use crate::sched::layout::{intr_buf, intr_qh, intr_td, INTR_BUF_LEN};
use crate::sched::qh::{start, stop};
use crate::sched::td::{state, Target, TdState};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntrEp {
    pub t: Target,
    /// Which of the INTR_MAX queues, TDs and buffers is this endpoint's.
    pub index: usize,
    pub len: usize,
    pub toggle: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Poll {
    Pending,
    Report(usize),
    Stalled,
    Error(u32),
}

pub fn arm<M: Mem>(m: &M, ep: &IntrEp) {
    let (td, buf) = (intr_td(ep.index), intr_buf(ep.index));
    let len = ep.len.min(INTR_BUF_LEN);
    write(m, td, ep.t, &[Step { pid: PID_IN, toggle: ep.toggle, buf, len, spd: false }]);
    start(m, intr_qh(ep.index), td);
}

pub fn poll<M: Mem>(m: &M, ep: &mut IntrEp, out: &mut [u8]) -> Poll {
    let td = intr_td(ep.index);
    match state(m.rd(td + 4)) {
        TdState::Active => Poll::Pending,
        TdState::Done(n) => {
            let n = n.min(out.len()).min(INTR_BUF_LEN);
            m.read_bytes(intr_buf(ep.index), &mut out[..n]);
            ep.toggle = !ep.toggle;
            arm(m, ep);
            Poll::Report(n)
        }
        TdState::Failed(bits) => {
            stop(m, intr_qh(ep.index));
            match bits & STATUS_STALLED {
                0 => Poll::Error(bits),
                _ => Poll::Stalled,
            }
        }
    }
}
