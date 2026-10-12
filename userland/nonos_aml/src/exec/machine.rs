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

//! The interpreter's state for one evaluation.

use super::{ExecLimits, Fault, FaultKind, Frame, Host};
use crate::load::Aml;
use crate::namespace::Id;
use alloc::vec::Vec;

/// One evaluation: a method run on the capsule's behalf, or the load-time
/// code of one table. Everything it may spend is counted here, and every
/// frame it pushes is popped on every path out, faults included.
pub struct Machine<'h, H: Host> {
    pub(crate) host: &'h mut H,
    pub limits: ExecLimits,
    pub(crate) eval: u64,
    pub(crate) steps: u64,
    pub(crate) alloc: usize,
    pub(crate) time_us: u64,
    /// Where the stack stood when the evaluation began (`stack.rs`).
    pub(crate) stack_top: usize,
    pub(crate) frames: Vec<Frame>,
    /// Methods running, nested inside one another.
    pub(crate) calls: u32,
    /// Mutexes this evaluation holds, oldest first, with their sync levels.
    pub(crate) held: Vec<(Id, u8)>,
}

impl<'h, H: Host> Machine<'h, H> {
    pub fn new(aml: &mut Aml, host: &'h mut H, limits: ExecLimits) -> Self {
        aml.rt.evaluations += 1;
        let eval = aml.rt.evaluations;
        let (frames, held, stack_top) = (Vec::new(), Vec::new(), super::stack::here());
        let (steps, alloc, time_us, calls) = (0, 0, 0, 0);
        Machine { host, limits, eval, steps, alloc, time_us, stack_top, frames, calls, held }
    }

    /// Steps taken so far, for the capsule's log.
    pub fn steps(&self) -> u64 {
        self.steps
    }

    pub(crate) fn frame(&self) -> &Frame {
        // Every evaluation pushes a frame before it reads a term.
        self.frames.last().expect("a frame is pushed before any term runs")
    }

    pub(crate) fn fault(&self, kind: FaultKind, at: u32) -> Fault {
        Fault::new(kind, self.frames.last().map_or(0, |f| f.table), at)
    }
}
