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

//! Taking every grant a process, or a process on one device, holds.

extern crate alloc;

use alloc::vec::Vec;

use super::super::grant::IrqGrant;
use super::store::RECORDS;

pub(in super::super) fn drain_for_pid(pid: u32) -> Vec<IrqGrant> {
    let mut all = RECORDS.lock();
    let mut taken = Vec::new();
    all.retain(|g| {
        if g.pid == pid {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}

pub(in super::super) fn drain_for_device(pid: u32, device_id: u64) -> Vec<IrqGrant> {
    let mut all = RECORDS.lock();
    let mut taken = Vec::new();
    all.retain(|g| {
        if g.pid == pid && g.device_id == device_id {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}
