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
//! `nonos-smp-shootdown-proof` only: the two round shapes a boot never shows
//! by itself, run once on the boot CPU right after the APs are up, before any
//! address space exists. A full flush (pages=0) to every AP, and a user-asid
//! round that must reach the one AP holding that asid and no other.

#[cfg(any(feature = "nonos-production", feature = "nonos-release"))]
compile_error!("nonos-smp-shootdown-proof is a boot proof and never part of a production build.");

use core::sync::atomic::Ordering;

use super::flush::{flush_tlb_all_smp, flush_tlb_one_smp};
use super::request::ASID_KERNEL;
use super::select::select;
use crate::memory::addr::VirtAddr;
use crate::smp::percpu::ASID_NONE;

/// Asids the allocator, counting up from `FIRST_USER_ASID`, cannot have
/// reached before userspace starts; the AP holds `HELD`, nobody holds `UNHELD`.
const HELD: u32 = u32::MAX;
const UNHELD: u32 = u32::MAX - 1;
/// A user address nothing maps yet; flushing it costs only the flush.
const USER_VA: u64 = 0x40_0000;

/// The AP is idle with no process to switch to, so nothing else writes its
/// asid in this window; the store stands in for its own CR3 switch, and the
/// rounds below test the filter and the delivery, not the switch.
pub fn run() {
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SHOOTDOWN-PROOF] ");
    flush_tlb_all_smp(ASID_KERNEL);
    let ap = (1..crate::smp::MAX_CPUS).find(|&c| crate::smp::cpu_is_online(c));
    let Some((cpu, d)) = ap.and_then(|c| crate::smp::percpu::get(c).map(|d| (c, d))) else {
        l.str(b"FAIL no AP online").end();
        return;
    };
    if d.active_asid.compare_exchange(ASID_NONE, HELD, Ordering::AcqRel, Ordering::Acquire).is_err()
    {
        l.str(b"FAIL ap=").dec(cpu as u64).str(b" already holds an asid").end();
        return;
    }
    let (held, held_targets, _) = select(HELD);
    flush_tlb_one_smp(VirtAddr::new(USER_VA), HELD);
    let (_, unheld_targets, _) = select(UNHELD);
    let restored = d
        .active_asid
        .compare_exchange(HELD, ASID_NONE, Ordering::AcqRel, Ordering::Acquire)
        .is_ok();
    let mut only = [0u64; super::select::WORDS];
    only[cpu / 64] = 1u64 << (cpu % 64);
    let pass = held == only && held_targets == 1 && unheld_targets == 0 && restored;
    l.str(b"full_flush=sent ap=").dec(cpu as u64);
    l.str(b" held_targets=").dec(u64::from(held_targets));
    l.str(b" unheld_targets=").dec(u64::from(unheld_targets));
    l.str(b" restored=").dec(u64::from(restored));
    l.str(if pass { b" PASS" } else { b" FAIL" });
    l.end();
}
