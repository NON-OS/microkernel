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

//! Running a written chain on a queue and watching it end: at its last TD,
//! at the first TD that failed, or at a short IN packet, which stops the
//! queue there (SPD). The queue is emptied before returning, so the
//! controller runs none of the chain after.

use super::chain::asked;
use crate::env::{Clock, Deadline, Mem};
use crate::facts::td::{STATUS_STALLED, TD_LEN};
use crate::sched::qh::{start, stop};
use crate::sched::td::{state, TdState};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ran {
    /// Every TD up to `upto` ran; `short` when the last of them was short.
    Ended {
        upto: usize,
        short: bool,
    },
    /// The endpoint refused: STALL.
    Stalled,
    /// A bus error on a TD: its status bits.
    Error(u32),
    TimedOut,
}

pub fn run<M: Mem, C: Clock>(m: &M, c: &C, qh: usize, base: usize, n: usize, ms: u64) -> Ran {
    start(m, qh, base);
    let d = Deadline::after(c, ms);
    let ran = loop {
        if let Some(r) = look(m, base, n) {
            break r;
        }
        if d.passed(c) {
            break look(m, base, n).unwrap_or(Ran::TimedOut);
        }
        c.idle();
    };
    stop(m, qh);
    ran
}

/// One look at the chain: None while it still runs.
pub fn look<M: Mem>(m: &M, base: usize, n: usize) -> Option<Ran> {
    for i in 0..n {
        let td = base + i * TD_LEN;
        match state(m.rd(td + 4)) {
            TdState::Active => return None,
            TdState::Failed(bits) if bits & STATUS_STALLED != 0 => return Some(Ran::Stalled),
            TdState::Failed(bits) => return Some(Ran::Error(bits)),
            TdState::Done(got) if got < asked(m, td) => {
                return Some(Ran::Ended { upto: i, short: true })
            }
            TdState::Done(_) => {}
        }
    }
    Some(Ran::Ended { upto: n - 1, short: false })
}
