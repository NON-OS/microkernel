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

use core::sync::atomic::Ordering::{self, Acquire};

use crate::arch::aarch64::gic;
use crate::arch::aarch64::gic::irq_handlers;
use crate::hardware::broker::irq::types::IrqError;

use super::pending::{Entry, SLOTS};

// Disable line, drop capsule ownership at the GIC registry (owner-checked),
// then free the slot. Taken first by the grant id it was found under, which
// no later bind of the slot carries: one found stale changes nothing (L77).
fn tear_down(e: &'static Entry, id: u64) -> bool {
    if id == 0 || e.grant_id.compare_exchange(id, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return false;
    }
    let intid = e.intid.load(Ordering::Acquire);
    if intid != 0 {
        gic::disable_irq(intid);
        let _ = irq_handlers::unregister_for_capsule(intid);
    }
    e.pid.store(0, Ordering::Release);
    e.intid.store(0, Ordering::Release);
    true
}

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let e = super::pending_find::find_by_grant(grant_id).ok_or(IrqError::UnknownGrant)?;
    if e.pid.load(Ordering::Acquire) != pid {
        return Err(IrqError::NotHolder);
    }
    if e.message.load(Ordering::Acquire) {
        return super::message::unmap_vector(pid, grant_id);
    }
    tear_down(e, grant_id).then_some(()).ok_or(IrqError::UnknownGrant)
}

pub fn release_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    // A message bind carries no epoch yet, so it goes by pid and device (L71).
    let messages = super::message::release_where(|b| b.pid == pid && b.device_id == device_id);
    let at = |a: &core::sync::atomic::AtomicU64| a.load(Ordering::Acquire);
    let ours = |e: &Entry| e.pid.load(Ordering::Acquire) == pid && at(&e.device_id) == device_id;
    messages + release_lines(|e| ours(e) && at(&e.claim_epoch) == epoch)
}

pub fn release_all_for_pid(pid: u32) -> usize {
    let messages = super::message::release_where(|b| b.pid == pid);
    messages + release_lines(|e| e.pid.load(Ordering::Acquire) == pid)
}

/// Tear down the line grants `which` picks (message vectors go with their
/// bind), each id read before the match, so a slot reused after fails it.
fn release_lines(which: impl Fn(&Entry) -> bool) -> usize {
    let line = |e: &&Entry| e.intid.load(Acquire) != 0 && !e.message.load(Acquire);
    let take = |e: &&'static Entry| {
        let id = e.grant_id.load(Acquire);
        which(e) && tear_down(e, id)
    };
    SLOTS.iter().filter(line).filter(take).count()
}
