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

//! Syscall-context half of `MkIrqWait` on the GIC. Arming stores the pid
//! in each grant's waiter word, which the trampoline swaps out and wakes;
//! the returned sequence is the sum of the grants' delivery counters, so a
//! delivery that beat the arm shows as a moved sequence and no sleep.
//! `grant_id == 0` covers every grant the pid holds.

use core::sync::atomic::Ordering;

use crate::hardware::broker::irq::types::IrqError;

use super::pending::{Entry, SLOTS};

fn owned(pid: u32, grant_id: u64) -> Result<impl Iterator<Item = &'static Entry>, IrqError> {
    if grant_id != 0 {
        let e = super::pending_find::find_by_grant(grant_id).ok_or(IrqError::UnknownGrant)?;
        if e.pid.load(Ordering::Acquire) != pid {
            return Err(IrqError::NotHolder);
        }
    } else if !SLOTS.iter().any(|e| held(e, pid)) {
        return Err(IrqError::UnknownGrant);
    }
    Ok(SLOTS.iter().filter(move |e| {
        held(e, pid) && (grant_id == 0 || e.grant_id.load(Ordering::Acquire) == grant_id)
    }))
}

fn held(e: &Entry, pid: u32) -> bool {
    e.intid.load(Ordering::Acquire) != 0 && e.pid.load(Ordering::Acquire) == pid
}

pub fn wait_arm(pid: u32, grant_id: u64) -> Result<u64, IrqError> {
    let mut seq = 0u64;
    for e in owned(pid, grant_id)? {
        e.waiter.store(pid, Ordering::Release);
        seq = seq.wrapping_add(e.pending.load(Ordering::Acquire));
    }
    Ok(seq)
}

pub fn wait_disarm(pid: u32, grant_id: u64) -> Result<u64, IrqError> {
    let mut seq = 0u64;
    for e in owned(pid, grant_id)? {
        let _ = e.waiter.compare_exchange(pid, 0, Ordering::AcqRel, Ordering::Relaxed);
        seq = seq.wrapping_add(e.pending.load(Ordering::Acquire));
    }
    Ok(seq)
}
