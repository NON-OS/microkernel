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

//! Waiting for an interrupt in the deepest state that is safe to enter.

use core::arch::asm;
use core::sync::atomic::Ordering;

use super::budget::budget_us;
use super::format::is_standby;
use super::powerdown;
use super::select::pick;
use super::table::{EXTENDED, REFUSED, TREE};
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::cpu::mpidr_affinity;
use crate::arch::aarch64::psci;

/// Wait for an interrupt and take it. Called and returns with IRQs masked.
pub fn wait() {
    let Some((cpu, index, param, standby)) = chosen() else {
        // `wfi` with IRQs open: a no-op when a wake is already pending, so
        // none is lost between unmasking and waiting.
        unsafe { asm!("msr daifclr, #2", "wfi", "msr daifset, #2", options(nomem, nostack)) };
        return;
    };
    // Entered masked, as a WFI may be: an IRQ pending at the GIC still ends
    // it (recalled, unverified), and is taken below. A standby state keeps
    // this CPU's context, and CPU_SUSPEND returns on the wake, ignoring the
    // entry point and context id (DEN0022). A powerdown state returns
    // through the resume path, or not at all if the core stayed up.
    let entered = if standby {
        psci::cpu_suspend(u64::from(param), 0, 0).map(|()| false)
    } else {
        powerdown::enter(cpu, param)
    };
    if entered.is_err() {
        if let Some(refused) = REFUSED.get(cpu) {
            refused.fetch_or(1u8.checked_shl(index.into()).unwrap_or(0), Ordering::Relaxed);
        }
        unsafe { asm!("wfi", options(nomem, nostack)) };
    }
    // The ISB is a context synchronization event, at which a pending,
    // now unmasked interrupt is taken before the mask goes back.
    unsafe { asm!("msr daifclr, #2", "isb", "msr daifset, #2", options(nomem, nostack)) };
}

/// This CPU's roster index, the state to enter, its power_state, and
/// whether it is a standby state.
fn chosen() -> Option<(usize, u8, u32, bool)> {
    let extended = *EXTENDED.get()?;
    let tree = TREE.get()?;
    let cpu = roster::index_of(mpidr_affinity())?;
    let list = tree.cpus.get(cpu)?.indices();
    let refused = REFUSED.get(cpu)?.load(Ordering::Relaxed);
    let powerdown = powerdown::available();
    let index = pick(tree.states(), list, extended, refused, budget_us(), powerdown)?;
    let param = tree.states().get(usize::from(index))?.param;
    Some((cpu, index, param, is_standby(param, extended)))
}
