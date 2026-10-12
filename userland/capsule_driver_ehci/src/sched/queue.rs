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

//! A QH's run: its qTDs handed over through the overlay's next pointer
//! (the overlay is empty, so the controller fetches the first on its next
//! visit), then watched until the last retires or one halts. One that
//! outlasts its bound is taken back (take.rs).

use super::take::take_back;
use crate::env::{until, Clock, Mem, Regs};
use crate::facts::qh::QH_NEXT;
use crate::facts::qtd::{TOK_ACTIVE, TOK_ERRORS, TOK_HALTED};
use crate::sched::qtd::token;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ran {
    Done,
    /// A qTD halted: its status bits.
    Halted(u32),
    TimedOut,
}

pub fn start<M: Mem>(m: &M, at: usize, first: usize) {
    m.wr(at + QH_NEXT, m.bus(first));
}

/// One look at the qTDs: None while one is still active.
pub fn look<M: Mem>(m: &M, qtds: &[usize]) -> Option<Ran> {
    let toks = qtds.iter().map(|&q| token(m, q));
    if let Some(t) = toks.clone().find(|t| t & TOK_HALTED != 0) {
        return Some(Ran::Halted(t & (TOK_ERRORS | TOK_HALTED)));
    }
    toks.clone().all(|t| t & TOK_ACTIVE == 0).then_some(Ran::Done)
}

/// Wait for ring QH `i`'s qTDs; past `ms` the QH is taken back.
pub fn wait<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    op: u32,
    i: usize,
    qtds: &[usize],
    ms: u64,
) -> Ran {
    let mut ran = None;
    until(c, ms, || {
        ran = look(m, qtds);
        ran.is_some()
    });
    ran.unwrap_or_else(|| {
        take_back((m, r, c), op, i, true, None);
        Ran::TimedOut
    })
}
