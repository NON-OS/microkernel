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

//! What a round slow to finish gets: at the warning mark a report and a
//! second delivery by NMI, at the deadline a stop of the whole machine.

use core::sync::atomic::Ordering;

use super::nudge::nudge_outstanding;
use super::report::report_stuck;
use super::request::{REQ_PENDING_ACKS, SHOOTDOWN_TIMEOUT_MS, SHOOTDOWN_WARN_MS};

/// Once per round, at `SHOOTDOWN_WARN_MS`.
pub(super) fn warn_slow() {
    let outstanding = REQ_PENDING_ACKS.load(Ordering::Acquire);
    if outstanding == 0 {
        return;
    }
    /*
     * The vector has had its chance; whoever still owes is probably masked,
     * so the round goes again as an NMI.
     */
    let nudged = nudge_outstanding();
    let mut line = crate::sys::serial::Line::new();
    line.str(b"[SMP] tlb shootdown slow: acks outstanding=").dec(outstanding as u64);
    line.str(b" after ms=").dec(SHOOTDOWN_WARN_MS);
    line.str(b" nmi sent=").dec(nudged as u64);
    line.end();
}

/// Past `SHOOTDOWN_TIMEOUT_MS` with `outstanding` acks still owed.
pub(super) fn fail_timed_out(outstanding: u32) -> ! {
    let mut line = crate::sys::serial::Line::new();
    line.str(b"[FATAL] TLB shootdown timeout outstanding=").dec(outstanding as u64);
    line.str(b" ms=").dec(SHOOTDOWN_TIMEOUT_MS);
    line.end();
    report_stuck();
    crate::smp::send_panic_ipi();
    crate::arch::halt_loop();
}
