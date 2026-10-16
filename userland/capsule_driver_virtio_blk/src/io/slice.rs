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

//! One wait on the device's interrupt.

use nonos_libc::{mk_irq_wait, MK_IRQ_WAIT_TIMED_OUT};

use super::rearm::rearm;
use crate::queue::Fault;
use crate::transport::Transport;

/// Sleep until the interrupt sequence moves past `*seq`, for at most `ms`,
/// and move `*seq` to where it stands. An interrupt with the request still
/// pending (a late one from an earlier request, or another device on a
/// shared line) left the line masked: it is rearmed here. The sequence was
/// read before the rearm, so an interrupt the unmask lets through ends the
/// next wait, and the caller's used-ring check comes after the rearm, so a
/// completion the status read swallowed is still seen.
pub(super) fn slice(transport: Transport, irq: u64, seq: &mut u64, ms: u64) -> Result<(), Fault> {
    let mut out = *seq;
    let rc = mk_irq_wait(irq, *seq, ms, &mut out);
    if rc < 0 {
        return Err(Fault::Wait);
    }
    if rc != MK_IRQ_WAIT_TIMED_OUT && out != *seq {
        rearm(transport, irq)?;
    }
    *seq = out;
    Ok(())
}
