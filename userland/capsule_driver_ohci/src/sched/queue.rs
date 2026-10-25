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

//! An ED's queue run. The TDs are written at the place the queue rests on
//! (HeadP and TailP both name it, so the controller fetches none of them),
//! then TailP moves to the ED's other place: the controller runs from HeadP
//! until it reaches TailP, halts, or is taken back: then the queue rests
//! on the other place.

use super::take::{retired, settle};
use crate::env::{until, Clock, Mem, Regs};
use crate::facts::ed::{HEAD_H, PTR_MASK};
use crate::facts::regs::HC_COMMAND_STATUS;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ran {
    Done,
    /// A TD failed and the ED halted on the TD after it.
    Halted,
    TimedOut,
}

/// Hand `ed` its TDs: TailP to `tail`, and `filled` (CLF or BLF, 0 for
/// the periodic list) so the controller looks at its list.
pub fn start<M: Mem, R: Regs>(m: &M, r: &mut R, ed: usize, tail: usize, filled: u32) {
    m.wr(ed + 4, m.bus(tail));
    if filled != 0 {
        r.w32(HC_COMMAND_STATUS, filled);
    }
}

/// One look: None while the queue still runs.
pub fn look<M: Mem>(m: &M, ed: usize, tail: usize) -> Option<Ran> {
    let head = m.rd(ed + 8);
    if head & HEAD_H != 0 {
        return Some(Ran::Halted);
    }
    (head & PTR_MASK == m.bus(tail)).then_some(Ran::Done)
}

/// Wait for the run; one that outlasts `ms` is taken back (take.rs).
pub fn wait<M: Mem, R: Regs, C: Clock>(
    m: &M,
    r: &mut R,
    c: &C,
    ed: usize,
    tail: usize,
    ms: u64,
) -> Ran {
    let mut ran = None;
    until(c, ms, || {
        ran = look(m, ed, tail);
        ran.is_some()
    });
    let ran = ran.unwrap_or_else(|| {
        settle(m, r, c, ed, true);
        Ran::TimedOut
    });
    retired(r);
    ran
}
