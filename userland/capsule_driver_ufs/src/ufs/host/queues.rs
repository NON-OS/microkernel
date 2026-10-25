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

//! The two request queues: their base addresses given, the host seen ready
//! for both and for UIC commands, then both set running. The task
//! management queue is never used, but the host expects both to run.

use core::sync::atomic::{fence, Ordering};

use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::hci::*;

pub fn run<H: Hw>(hw: &H, transfer_bus: u64, task_bus: u64) -> Result<(), UfsError> {
    for (lo, hi, bus) in [(UTRLBA, UTRLBAU, transfer_bus), (UTMRLBA, UTMRLBAU, task_bus)] {
        hw.w32(lo, bus as u32);
        hw.w32(hi, (bus >> 32) as u32);
    }
    // The bases land before the ready check and the run bits.
    fence(Ordering::SeqCst);
    if hw.r32(HCS) & HCS_LISTS_READY != HCS_LISTS_READY {
        return Err(UfsError::Host);
    }
    hw.w32(UTMRLRSR, RUN);
    hw.w32(UTRLRSR, RUN);
    Ok(())
}
