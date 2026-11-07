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

//! Imports taken out by what ends them: the attach that held the frames, the
//! device's release or revoke, the holder's exit. Each leaves the device's
//! domain before the caller goes on to let the frames go.

extern crate alloc;

use alloc::vec::Vec;

use super::records::RECORDS;
use super::table_rule::{take_where, Import};

/// `pid` lets its attach of `handle` go: under the registry's gate.
pub fn drop_for_attach(pid: u32, handle: u64) {
    let gone = take_where(&mut RECORDS.lock(), |i| i.pid == pid && i.handle == handle);
    take_all(gone);
}

/// The surface `handle` is gone, every holder with it.
pub fn drop_for_handle(handle: u64) {
    let gone = take_where(&mut RECORDS.lock(), |i| i.handle == handle);
    take_all(gone);
}

/// `device_id`, held by `pid`, is released or revoked.
pub(in crate::hardware::broker) fn drop_for_device(pid: u32, device_id: u64) {
    let gone = take_where(&mut RECORDS.lock(), |i| i.pid == pid && i.device_id == device_id);
    take_all(gone);
}

/// The holder `pid` exits.
pub fn drop_for_pid(pid: u32) {
    let gone = take_where(&mut RECORDS.lock(), |i| i.pid == pid);
    take_all(gone);
}

fn take_all(gone: Vec<Import>) {
    for i in &gone {
        take_out(i);
    }
}

/// Out of the device's domain, the invalidation waited on; or, if the unit
/// will not confirm that, the frames kept from the allocator for good.
pub(super) fn take_out(i: &Import) {
    if !crate::hardware::broker::confine::unmap(i.pid, i.iova, i.length, true) {
        quarantine(&i.runs);
    }
}

pub(super) fn quarantine(runs: &[(u64, u64)]) {
    crate::sys::serial::println(b"[DMA] imported surface still reachable; frames quarantined");
    for &(phys, len) in runs {
        crate::memory::phys::quarantine::hold(phys, len);
    }
}
