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
//! One sliced wait on the device's interrupt.

use super::error::BlkError;
use super::rearm::rearm;
use crate::regs::Regs;
use nonos_libc::{mk_irq_wait, MK_IRQ_WAIT_TIMED_OUT};

/// The longest one wait sleeps.
const WAIT_SLICE_MS: u64 = 100;

pub(super) enum Slice {
    /// An interrupt came; the sequence it left.
    Woke(u64),
    /// The whole slice passed with no interrupt (the kernel says so with
    /// `MK_IRQ_WAIT_TIMED_OUT`). Before the kernel told a timeout apart from
    /// a wake this case was never seen and only the pass guard bounded a dead
    /// device, at 5000 slices (over eight minutes) instead of five seconds.
    TimedOut,
    /// The wait itself failed.
    Failed,
}

/// Wait up to one slice for the interrupt sequence to move past `seq`.
///
/// An interrupt with the request still pending (a late one from the previous
/// request, or another device on the line) left the line masked: it is
/// rearmed here. `out_seq` was read before the rearm, so an interrupt the
/// unmask lets through ends the next wait. The caller's used-ring check comes
/// after the rearm, so a completion the status read swallowed is still seen.
pub(super) fn wait_slice(regs: Regs, irq_grant: u64, seq: u64) -> Result<Slice, BlkError> {
    let mut out_seq: u64 = seq;
    let rc = mk_irq_wait(irq_grant, seq, WAIT_SLICE_MS, &mut out_seq);
    if rc < 0 {
        return Ok(Slice::Failed);
    }
    if rc == MK_IRQ_WAIT_TIMED_OUT {
        return Ok(Slice::TimedOut);
    }
    if out_seq != seq {
        rearm(regs, irq_grant)?;
    }
    Ok(Slice::Woke(out_seq))
}
