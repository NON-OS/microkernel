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

use core::sync::atomic::Ordering;

use super::request::{REQ_PAGES, REQ_PENDING_ACKS, REQ_VA};

/// What every CPU looked like when the round gave up, printed before the halt.
/// A timeout says only that an ack did not arrive; which cpu owed it, whether
/// it was marked, and whether it is idle or in a handler separate "the IPI was
/// never delivered" from "the cpu was in no position to run it". The head
/// line's va and pages (0 for a whole flush) name the change that waited.
pub(super) fn report_stuck() {
    let mut head = crate::sys::serial::Line::new();
    head.str(b"[SMP] acks outstanding=").dec(REQ_PENDING_ACKS.load(Ordering::Acquire) as u64);
    head.str(b" va=").hex(REQ_VA.load(Ordering::Acquire));
    head.str(b" pages=").dec(REQ_PAGES.load(Ordering::Acquire) as u64);
    head.end();
    for cpu in 0..crate::smp::MAX_CPUS {
        // A cpu still starting is a target of every kernel round (`select`),
        // and the one most likely to owe the ack, so it is listed too.
        let starting = crate::smp::cpu_is_starting(cpu);
        if !crate::smp::cpu_is_online(cpu) && !starting {
            continue;
        }
        super::report_cpu::line(cpu, starting);
    }
}
