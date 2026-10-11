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

//! ufshcd_make_hba_operational: both list bases written, the host seen ready
//! for both lists and for UIC commands, then both lists set running. The
//! task management list is never used, but the host runs it with the other.

use super::regs::*;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;

pub fn run<H: Hw>(hw: &H, utrl_bus: u64, utmrl_bus: u64) -> Result<(), UfsError> {
    hw.w32(UTRLBA, utrl_bus as u32);
    hw.w32(UTRLBAU, (utrl_bus >> 32) as u32);
    hw.w32(UTMRLBA, utmrl_bus as u32);
    hw.w32(UTMRLBAU, (utmrl_bus >> 32) as u32);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    if hw.r32(HCS) & HCS_LISTS_READY != HCS_LISTS_READY {
        return Err(UfsError::Host);
    }
    hw.w32(UTMRLRSR, RUN);
    hw.w32(UTRLRSR, RUN);
    Ok(())
}
