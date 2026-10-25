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

use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::{AcqRel, Acquire, Release};

use super::flush::{flush_tlb_all_smp, flush_tlb_one_smp};
use super::request::ASID_KERNEL;
use super::select::select;
use crate::memory::addr::VirtAddr;
use crate::smp::percpu::ASID_NONE;

/// Asids the allocator, counting up from `FIRST_USER_ASID`, cannot have
/// reached before userspace starts; the AP holds `HELD`, nobody holds `UNHELD`.
/// `USER_VA` is a user address nothing maps yet.
const HELD: u32 = u32::MAX;
const UNHELD: u32 = u32::MAX - 1;
const USER_VA: u64 = 0x40_0000;
/// Set while the rounds below run, so the trace prints them.
pub(super) static RUNNING: AtomicBool = AtomicBool::new(false);

/// The AP is idle with nothing to switch to, so only this writes its asid; the
/// store stands in for its CR3 switch: the filter and delivery are proved, not that.
pub fn run() {
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SHOOTDOWN-PROOF] ");
    RUNNING.store(true, Release);
    flush_tlb_all_smp(ASID_KERNEL);
    rounds(&mut l);
    RUNNING.store(false, Release);
    l.end();
}

fn rounds(l: &mut crate::sys::serial::Line) {
    let ap = (1..crate::smp::MAX_CPUS).find(|&c| crate::smp::cpu_is_online(c));
    let Some((cpu, d)) = ap.and_then(|c| crate::smp::percpu::get(c).map(|d| (c, d))) else {
        l.str(b"FAIL no AP online");
        return;
    };
    let asid = &d.active_asid;
    if asid.compare_exchange(ASID_NONE, HELD, AcqRel, Acquire).is_err() {
        l.str(b"FAIL ap=").dec(cpu as u64).str(b" already holds an asid");
        return;
    }
    let (held, held_targets, _) = select(HELD);
    flush_tlb_one_smp(VirtAddr::new(USER_VA), HELD);
    let (_, unheld_targets, _) = select(UNHELD);
    let restored = asid.compare_exchange(HELD, ASID_NONE, AcqRel, Acquire).is_ok();
    let only: [u64; super::select::WORDS] =
        core::array::from_fn(|w| if w == cpu / 64 { 1 << (cpu % 64) } else { 0 });
    let pass = held == only && held_targets == 1 && unheld_targets == 0 && restored;
    l.str(b"full_flush=sent ap=").dec(cpu as u64).str(b" held_targets=");
    l.dec(u64::from(held_targets)).str(b" unheld_targets=").dec(u64::from(unheld_targets));
    l.str(b" restored=").dec(u64::from(restored)).str(if pass { b" PASS" } else { b" FAIL" });
}
