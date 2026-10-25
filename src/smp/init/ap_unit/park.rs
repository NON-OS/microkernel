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

//! Parking an AP that claimed its start and missed its online deadline.

use super::report::{abandoned, report, stuck};
use super::wait::{wait_for, AP_PARK_TIMEOUT_MS};
use crate::smp::{CpuDescriptor, CpuState};

/*
 * An AP that entered and missed its online deadline is running kernel code
 * on the kernel's tables, and every kernel shootdown waits for it
 * (`shootdown::select` targets `Starting`). It is asked to park at its next
 * checkpoint, where it holds no lock, and only once it has may INIT reset it;
 * then it is `Offline`, never started again this boot, and the shootdown
 * waiter answers for it. Until then it stays `Starting`, and writing `Offline`
 * would be wrong: a running CPU recorded as down is skipped by shootdowns and
 * keeps stale entries. So an AP that does not park in time, or whose INIT
 * fails, stays a target, and the next kernel shootdown's deadline stops the
 * machine rather than leave it unflushed. One that reaches a checkpoint only
 * after this wait has given up still parks, halted with interrupts masked,
 * but is never reset, and stays a target too.
 */
pub(super) fn park(ap: &CpuDescriptor, cpu_id: usize, apic_id: u32) -> bool {
    if !ap.boot_claim.ask_park() {
        // Only an AP that entered gets here, and it is asked once.
        stuck(cpu_id, b"its claim was not entered; it stays a shootdown target");
        return false;
    }
    let answered =
        wait_for(|| ap.boot_claim.parked() || ap.state() == CpuState::Online, AP_PARK_TIMEOUT_MS);
    if ap.state() == CpuState::Online {
        // It passed its last checkpoint before the request and went live.
        report(cpu_id, apic_id, b" online, late", b"");
        return true;
    }
    if !answered || !ap.boot_claim.parked() {
        stuck(cpu_id, b"it did not park within 100 ms; it stays a shootdown target");
        return false;
    }
    match super::abandon::reset(apic_id) {
        Ok(()) => {
            // Reset by INIT: halted in wait-for-SIPI with an empty TLB.
            ap.set_state(CpuState::Offline);
            abandoned(cpu_id, b"missed its online deadline; parked and reset by INIT");
        }
        Err(e) => {
            let mut l = crate::sys::serial::Line::new();
            l.str(b"[SMP] cpu ").dec(cpu_id as u64);
            l.str(b" stuck at bring-up: parked, INIT failed: ").str(e.as_bytes());
            l.str(b"; halted with interrupts masked, it stays a shootdown target");
            l.end();
        }
    }
    false
}
