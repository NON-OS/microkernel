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

//! One look at an interrupt endpoint's armed TD.

use super::interrupt::{arm, IntrEp};
use crate::env::{Mem, Regs};
use crate::sched::layout::{intr_buf, intr_ed, intr_td, INTR_BUF_LEN};
use crate::sched::queue::{look, Ran};
use crate::sched::take::clear_halt;
use crate::sched::take::retired;
use crate::sched::td::moved;
use crate::xfer::failed::{failed, CtrlError};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Poll {
    Pending,
    Report(usize),
    Stalled,
    Error(u32),
}

pub fn poll<M: Mem, R: Regs>(m: &M, r: &mut R, ep: &mut IntrEp, out: &mut [u8]) -> Poll {
    let (i, here) = (ep.index, intr_td(ep.index, ep.at));
    let got = match look(m, intr_ed(i), intr_td(i, 1 - ep.at)) {
        None => return Poll::Pending,
        Some(Ran::Done) => {
            let n = moved(m, here, intr_buf(i), ep.len.min(INTR_BUF_LEN)).min(out.len());
            m.read_bytes(intr_buf(i), &mut out[..n]);
            Poll::Report(n)
        }
        Some(_) => {
            // The halt cleared with DATA0 next, as after the class driver's
            // CLEAR_FEATURE(ENDPOINT_HALT).
            clear_halt(m, intr_ed(i), false);
            match failed(m, here, 1) {
                CtrlError::Stalled => Poll::Stalled,
                CtrlError::Bus(cc) => Poll::Error(cc),
                _ => Poll::Error(0),
            }
        }
    };
    retired(r);
    ep.at = 1 - ep.at;
    arm(m, r, ep);
    got
}
