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

use crate::arch::aarch64::gic;
use crate::arch::aarch64::gic::irq_handlers;
use crate::hardware::broker::irq::types::IrqError;

use super::pending::{Entry, SLOTS};

// Disable line, drop capsule ownership at the GIC registry, then free
// the broker slot. `unregister_for_capsule` is owner-checked so a
// kernel-claimed line can never be released through this path.
fn tear_down(e: &'static Entry) {
    let intid = e.intid.load(Ordering::Acquire);
    if intid != 0 {
        gic::disable_irq(intid);
        let _ = irq_handlers::unregister_for_capsule(intid);
    }
    e.intid.store(0, Ordering::Release);
    e.pid.store(0, Ordering::Release);
    e.grant_id.store(0, Ordering::Release);
}

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let e = super::pending_find::find_by_grant(grant_id).ok_or(IrqError::UnknownGrant)?;
    if e.pid.load(Ordering::Acquire) != pid {
        return Err(IrqError::NotHolder);
    }
    if e.message.load(Ordering::Acquire) {
        return super::message::unmap_vector(pid, grant_id);
    }
    tear_down(e);
    Ok(())
}

pub fn release_for_device(pid: u32, device_id: u64) -> usize {
    let messages = super::message::release_where(|b| b.pid == pid && b.device_id == device_id);
    let lines = release_lines(|e| {
        e.pid.load(Ordering::Acquire) == pid && e.device_id.load(Ordering::Acquire) == device_id
    });
    messages + lines
}

pub fn release_all_for_pid(pid: u32) -> usize {
    let messages = super::message::release_where(|b| b.pid == pid);
    messages + release_lines(|e| e.pid.load(Ordering::Acquire) == pid)
}

/// Tear down the line grants `which` picks; message vectors go with their
/// bind, through its own teardown.
fn release_lines(which: impl Fn(&Entry) -> bool) -> usize {
    let live = |e: &&Entry| e.intid.load(Ordering::Acquire) != 0;
    let lines = SLOTS.iter().filter(live).filter(|e| !e.message.load(Ordering::Acquire));
    lines.filter(|e| which(e)).map(tear_down).count()
}
