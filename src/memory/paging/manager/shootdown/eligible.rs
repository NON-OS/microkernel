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

//! Which cpus a round must reach: the two tests `select` applies to each.

use core::sync::atomic::Ordering;

use super::request::ASID_KERNEL;
use crate::smp::percpu::ASID_NONE;

/// Online, or, for a kernel round, still being brought up.
///
/// An AP runs kernel code on the kernel's CR3 from the trampoline on, long
/// before it publishes `Online`, and caches what it touches. Targeting only
/// `Online` skipped it: a round could return while an AP in `bring_up` still
/// held the old entry, and the page under it could be freed and reused. The
/// boot CPU stores `Starting` before it sends INIT (`ap_unit::start`), and
/// INIT empties the AP's TLB, so a cpu read here as `Offline` holds nothing
/// from before this round's write. A `Starting` target takes no IPI
/// (`send.rs`) and serves its mark itself; one the boot CPU gives up on is
/// answered for by the waiter (`wait.rs`). `userland/shootdown_proofs`
/// searches every interleaving of this with the round and its mutants.
///
/// A user round needs no such cpu: bring-up runs on the kernel's tables,
/// which map no user address space, so its active asid is none.
#[inline]
pub(super) fn may_hold_entries(cpu: usize, asid: u32) -> bool {
    crate::smp::cpu_is_online(cpu) || (asid == ASID_KERNEL && crate::smp::cpu_is_starting(cpu))
}

/// Only a cpu running the asid can hold its entries: CR3 is loaded untagged
/// (PCID 0), which drops every non-global entry, so a cpu that switched away
/// holds none. With PCIDs this must become every cpu that has run the asid
/// since its last flush, or a tagged stale entry survives the switch back.
#[inline]
pub(super) fn cpu_should_flush(data: &crate::smp::percpu::PerCpuData, asid: u32) -> bool {
    if asid == ASID_KERNEL {
        return true;
    }
    let active = data.active_asid.load(Ordering::Acquire);
    active != ASID_NONE && active == asid
}
