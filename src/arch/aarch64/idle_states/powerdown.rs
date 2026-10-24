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

//! A powerdown idle state: the context armed, the call made, the outcome.

use super::context::{resume_lost, SuspendContext, SLOTS};
use super::flags::flags;
use super::layout::{ARMED, FLAGS, MAGIC, MPIDR, RESUMED};
use super::resume_map;
use crate::arch::aarch64::cpu::mpidr_affinity;
use crate::arch::aarch64::mmu::image_pa;
use crate::arch::aarch64::psci::{conduit, PsciError, PsciMethod};

extern "C" {
    fn aarch64_cpu_suspend(
        ctx: *mut SuspendContext,
        state: u64,
        entry: u64,
        id: u64,
        smc: u64,
    ) -> i64;
    fn _aarch64_resume_start();
}

/// Whether a powerdown state may be asked for at all: there is firmware to
/// ask, and no core has come back somewhere it could not return from.
pub(super) fn available() -> bool {
    conduit().is_some() && !resume_lost() && resume_map::verified()
}

/// Enter powerdown state `param` on roster CPU `cpu`, the caller. True when
/// the core went down and came back, false when it stayed up, an error when
/// PSCI refused. Called with IRQs masked, which it returns with.
pub(super) fn enter(cpu: usize, param: u32) -> Result<bool, PsciError> {
    let via = conduit().ok_or(PsciError::NotPresent)?;
    let ctx = SLOTS.get(cpu).ok_or(PsciError::InvalidParams)?;
    // SAFETY: this CPU's own slot, with IRQs masked (context.rs).
    unsafe {
        (*ctx).words[MAGIC] = ARMED;
        (*ctx).words[MPIDR] = mpidr_affinity();
        (*ctx).words[FLAGS] = flags();
    }
    let entry = image_pa(_aarch64_resume_start as unsafe extern "C" fn() as u64);
    let smc = u64::from(via == PsciMethod::Smc);
    // SAFETY: suspend.S keeps AAPCS64 whichever way the call goes: it
    // returns once, on this stack, with the callee-saved registers, the
    // FP/SIMD registers when they were live, and this CPU's EL1 state as
    // they were. Firmware is handed the context's physical address.
    let ret =
        unsafe { aarch64_cpu_suspend(ctx, u64::from(param), entry, image_pa(ctx as u64), smc) };
    // SAFETY: as above. A resume has disarmed it already; a return has not.
    unsafe { (*ctx).words[MAGIC] = 0 };
    match ret {
        RESUMED => Ok(true),
        other => PsciError::from_ret(other as i32).map(|()| false),
    }
}
