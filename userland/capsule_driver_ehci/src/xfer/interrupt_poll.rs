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

//! One look at an interrupt endpoint's armed qTD. A halted one (a STALL)
//! is emptied with DATA0 next, as after the class driver's
//! CLEAR_FEATURE(ENDPOINT_HALT), and armed again.

use super::failed::{failed, CtrlError};
use super::interrupt::{arm, IntrEp};
use crate::env::Mem;
use crate::sched::layout::{intr_buf, intr_qh, intr_qtd, INTR_BUF_LEN};
use crate::sched::qtd::moved;
use crate::sched::queue::{look, Ran};
use crate::sched::take::clear_halt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Poll {
    Pending,
    Report(usize),
    Stalled,
    Error(u32),
}

pub fn poll<M: Mem>(m: &M, ep: &mut IntrEp, out: &mut [u8]) -> Poll {
    let i = ep.index;
    let got = match look(m, &[intr_qtd(i)]) {
        None => return Poll::Pending,
        Some(Ran::Done) => {
            let n = moved(m, intr_qtd(i), ep.len.min(INTR_BUF_LEN)).min(out.len());
            m.read_bytes(intr_buf(i), &mut out[..n]);
            Poll::Report(n)
        }
        Some(Ran::Halted(status)) => {
            clear_halt(m, intr_qh(i), false);
            match failed(status) {
                CtrlError::Stalled => Poll::Stalled,
                _ => Poll::Error(status),
            }
        }
        Some(Ran::TimedOut) => Poll::Error(0),
    };
    arm(m, ep);
    got
}
