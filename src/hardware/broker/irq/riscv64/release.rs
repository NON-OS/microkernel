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
    let mut n = 0;
    for e in SLOTS.iter() {
        if e.source.load(Ordering::Acquire) == 0 {
            continue;
        }
        let ours = e.pid.load(Ordering::Acquire) == pid
            && e.device_id.load(Ordering::Acquire) == device_id
            && e.claim_epoch.load(Ordering::Acquire) == epoch;
        if ours && tear_down(e, e.grant_id.load(Ordering::Acquire)) {
            n += 1;
        }
    }
    n
}

pub fn release_all_for_pid(pid: u32) -> usize {
    let mut n = 0;
    for e in SLOTS.iter() {
        if e.source.load(Ordering::Acquire) == 0 {
            continue;
        }
        if e.pid.load(Ordering::Acquire) == pid && tear_down(e, e.grant_id.load(Ordering::Acquire))
        {
            n += 1;
        }
    }
    n
}
