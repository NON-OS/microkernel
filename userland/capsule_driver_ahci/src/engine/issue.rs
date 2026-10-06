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

use super::completion::{wait_done, wait_ready, SLOT0};
use crate::clock::Deadline;
use crate::constants::regs::{PORT_CI, PORT_IS};
use crate::constants::timing::COMMAND_MS;
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Issue the command built in slot 0 and wait for it, all inside one
/// COMMAND_MS budget so the reply reaches the kernel before it stops waiting.
/// A device still BSY or DRQ from before is kicked once (`recover`) and given
/// the rest of the budget.
pub(super) fn issue_slot0(regs: Regs, base: u32, sclo: bool) -> AhciResult<()> {
    let read = |off: u32| unsafe { regs.r32(base + off) };
    let deadline = Deadline::after_ms(COMMAND_MS);
    unsafe {
        regs.w32(base + PORT_IS, u32::MAX);
    }
    match wait_ready(read, || deadline.expired()) {
        Err(AhciError::Timeout) => {
            super::recover::recover(regs, base, sclo)?;
            wait_ready(read, || deadline.expired())?;
        }
        other => other?,
    }
    // The HBA DMA-reads the command list and table only after it sees the
    // command-issue bit set. Order those stores ahead of the CI write so it
    // never fetches a stale command header, FIS, or PRDT.
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    unsafe {
        regs.w32(base + PORT_CI, SLOT0);
    }
    wait_done(read, || deadline.expired())
}
