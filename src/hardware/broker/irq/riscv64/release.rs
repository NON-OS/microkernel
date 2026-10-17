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

use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::{self, Acquire};

use crate::arch::riscv64::plic;
use crate::arch::riscv64::plic::irq_handlers;
use crate::hardware::broker::irq::types::IrqError;

use super::pending::{Entry, SLOTS};

// Disable line, drop capsule ownership at the PLIC registry (owner-checked),
// then free the slot; taken first by its grant id, as on aarch64 (L77).
fn tear_down(e: &'static Entry, id: u64) -> bool {
    if id == 0 || e.grant_id.compare_exchange(id, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return false;
    }
    let source = e.source.load(Ordering::Acquire);
    if source != 0 {
        let _ = plic::disable_irq(source);
        let _ = irq_handlers::unregister_for_capsule(source);
    }
    e.pid.store(0, Ordering::Release);
    e.source.store(0, Ordering::Release);
    true
}

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let e = super::pending::find_by_grant(grant_id).ok_or(IrqError::UnknownGrant)?;
    if e.pid.load(Ordering::Acquire) != pid {
        return Err(IrqError::NotHolder);
    }
    tear_down(e, grant_id).then_some(()).ok_or(IrqError::UnknownGrant)
}

pub fn release_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    let at = |a: &AtomicU64| a.load(Acquire);
    let ours = |e: &Entry| e.pid.load(Acquire) == pid && at(&e.device_id) == device_id;
    release_lines(|e| ours(e) && at(&e.claim_epoch) == epoch)
}

pub fn release_all_for_pid(pid: u32) -> usize {
    release_lines(|e| e.pid.load(Acquire) == pid)
}

/// Tear down the grants `which` picks, each id read before the match, so a
/// slot reused after fails it.
fn release_lines(which: impl Fn(&Entry) -> bool) -> usize {
    let take = |e: &&'static Entry| {
        let id = e.grant_id.load(Acquire);
        e.source.load(Acquire) != 0 && which(e) && tear_down(e, id)
    };
    SLOTS.iter().filter(take).count()
}
