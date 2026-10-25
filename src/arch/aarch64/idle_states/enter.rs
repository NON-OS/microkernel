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

use super::allow::allow;
use super::budget::budget_us;
use super::call::{enter_state, Chosen};
use super::format::is_standby;
use super::select::pick;
use super::table::{EXTENDED, REFUSED, TREE};
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::cpu::mpidr_affinity;

/// Wait for an interrupt and take it. Called and returns with IRQs masked.
pub fn wait() {
    let Some(chosen) = chosen() else {
        // `wfi` with IRQs open: a no-op when a wake is already pending, so
        // none is lost between unmasking and waiting.
        unsafe { asm!("msr daifclr, #2", "wfi", "msr daifset, #2", options(nomem, nostack)) };
        return;
    };
    // Entered masked, as a WFI may be: an IRQ pending at the GIC still ends
    // it (recalled, unverified), and is taken below.
    if enter_state(&chosen).is_err() {
        if let Some(refused) = REFUSED.get(chosen.cpu) {
            let bit = 1u8.checked_shl(chosen.index.into()).unwrap_or(0);
            refused.fetch_or(bit, Ordering::Relaxed);
        }
        unsafe { asm!("wfi", options(nomem, nostack)) };
    }
    // The ISB is a context synchronization event, at which a pending,
    // now unmasked interrupt is taken before the mask goes back.
    unsafe { asm!("msr daifclr, #2", "isb", "msr daifset, #2", options(nomem, nostack)) };
}

/// The state this CPU may enter now, if any beats a plain WFI.
fn chosen() -> Option<Chosen> {
    let extended = *EXTENDED.get()?;
    let tree = TREE.get()?;
    let cpu = roster::index_of(mpidr_affinity())?;
    // The idle lists are in the tree's order, the index the roster's.
    let list = tree.cpus.get(roster::tree_index(cpu)?)?.indices();
    let refused = REFUSED.get(cpu)?.load(Ordering::Relaxed);
    let index = pick(tree.states(), list, extended, refused, budget_us(), allow(cpu))?;
    let state = *tree.states().get(usize::from(index))?;
    Some(Chosen { cpu, index, state, standby: is_standby(state.param, extended) })
}
