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

//! Taking every grant of a dying process, or of a released device, out of
//! the grant table at once.

extern crate alloc;

use alloc::vec::Vec;

use super::records::RECORDS;
use super::types::DmaGrant;

pub(super) fn drain_for_pid(pid: u32) -> Vec<DmaGrant> {
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

pub(super) fn drain_for_device(pid: u32, device_id: u64) -> Vec<DmaGrant> {
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
