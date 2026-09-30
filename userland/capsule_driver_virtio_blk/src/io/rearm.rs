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

//! Lower the device's interrupt line and let the next completion through.
//!
//! A legacy virtio device holds its INTx line up until the driver reads the
//! interrupt status register, and the line is level triggered. The kernel
//! masks the line in the interrupt it delivers; the ack unmasks it. Acked
//! with the status unread, the line was still up: it fired again at once,
//! masked itself again, and stayed masked through the next request, whose
//! completion then raised no interrupt at all. Each such request slept out
//! its whole wait slice. On one CPU the device usually finished before the
//! first used-ring check, so the wait was rare; with several CPUs it
//! finishes a moment later and every request waited 100 ms.

use super::error::BlkError;
use crate::constants::LEG_ISR;
use crate::regs::Regs;
use nonos_libc::mk_irq_ack;

/// Read (and so clear) the interrupt status, then unmask the line. With
/// MSI-X the status is unused and the ack is a no-op: both stay harmless.
pub(super) fn rearm(regs: Regs, irq_grant: u64) -> Result<(), BlkError> {
    let _ = unsafe { regs.r8(LEG_ISR) };
    if mk_irq_ack(irq_grant) < 0 {
        return Err(BlkError::Io);
    }
    Ok(())
}
